use super::common::prelude::*;
use super::common::setup_wtransport_client;
use super::GameEvent;
use anyhow::Result;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Clone, Resource)]
#[serde(default)]
pub struct NetSettings {
    pub enabled: bool,
    // TODO: do things via ToSocketAddrs to support DNS
    pub last_host_addr: String,
    pub last_host_sessionid: u32,
    pub worker: NetWorkerConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetWorkerConfig {
    pub ca_cert: PathBuf,
    pub host_client_cert: Vec<PathBuf>,
    pub host_client_key: PathBuf,
    // pub auth_client_cert: Vec<PathBuf>,
    // pub auth_client_key: PathBuf,
}

impl Default for NetSettings {
    fn default() -> Self {
        NetSettings {
            enabled: true,
            last_host_addr: "https://127.0.0.1".into(),
            last_host_sessionid: 0,
            worker: NetWorkerConfig {
                ca_cert: "cert/ca.pem".into(),
                host_client_cert: vec!["cert/hostclient.cert.pem".into(), "cert/ca.pem".into()],
                host_client_key: "cert/hostclient.key.pem".into(),
                // auth_client_cert: vec!["cert/authclient.cert.der".into(), "cert/ca.cert.der".into()],
                // auth_client_key: "cert/authclient.key.der".into(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum NetWorkerControl {
    Disconnect,
    ConnectHost(HostSessionConfig),
}

#[derive(Debug)]
pub enum NetWorkerStatus {
    NetError(anyhow::Error),
    NetDisabled,
    HostDisconnected,
    HostConnected,
    AuthDisconnected,
    AuthConnected,
}

#[derive(Debug, Clone)]
pub struct HostSessionConfig {
    pub addr: String,
    pub session_id: Option<u32>,
    pub server_name: Option<String>,
}

impl HostSessionConfig {
    fn server_name(&self) -> &str {
        if let Some(name) = &self.server_name {
            name.as_str()
        } else {
            // FIXME
            "auth.iyes.games"
            // "mw_generic_host"
        }
    }
}

pub type TxShutdown = TxBroadcast<()>;
pub type RxShutdown = RxBroadcast<()>;

pub struct Channels {
    pub tx_shutdown: TxShutdown,
    pub rx_shutdown: RxShutdown,
    pub tx_game_event: TxMpscU<GameEvent>,
    pub tx_status: TxMpscU<NetWorkerStatus>,
    pub rx_control: RxBroadcast<NetWorkerControl>,
}

impl Channels {
    fn net_disable(&mut self) {
        self.tx_status.send(NetWorkerStatus::NetDisabled).ok();
        self.tx_shutdown.send(()).ok();
    }
}

impl Clone for Channels {
    fn clone(&self) -> Self {
        Channels {
            tx_shutdown: self.tx_shutdown.clone(),
            rx_shutdown: self.rx_shutdown.resubscribe(),
            tx_game_event: self.tx_game_event.clone(),
            tx_status: self.tx_status.clone(),
            rx_control: self.rx_control.resubscribe(),
        }
    }
}

struct HostSessionState {
    connection: xwebtransport::current::Connection,
}

struct NetWorkerState {
    endpoint: xwebtransport::current::Endpoint,
}

async fn setup(config: &NetWorkerConfig) -> Result<NetWorkerState> {
    // let crypto = load_client_crypto(&config.ca_cert, false, &[""], "").await?;
    let endpoint = setup_wtransport_client("https://[::1]:4433")?;
    Ok(NetWorkerState { endpoint })
}

async fn connect_host(
    endpoint: &wtransport::Endpoint<endpoint_side::Client>,
    config: &HostSessionConfig,
) -> Result<HostSessionState> {
    info!("Connecting to Host: {}", config.addr);
    let connection = endpoint.connect("https://127.0.0.1:4433").await.unwrap();
    // let connection = connecting.await?;
    Ok(HostSessionState { connection })
}

async fn async_main(config: NetWorkerConfig, mut channels: Channels) {
    let mut state = match setup(&config).await {
        Ok(state) => state,
        Err(e) => {
            error!("Could not set up networking: {}", e);
            channels.tx_status.send(NetWorkerStatus::NetError(e)).ok();
            channels.net_disable();
            return;
        }
    };

    info!("OK: got init config done");

    loop {
        tokio::select! {
            _ = channels.rx_shutdown.recv() => {
                info!("Shutdown recv!");
                break;
            }
            Ok(control) = channels.rx_control.recv() => {
                info!("OK: got rx control recv");
                match control {
                    NetWorkerControl::ConnectHost(config) => {
                        match connect_host(&state.endpoint, &config).await {
                            Ok(session) => {
                                channels.tx_status.send(NetWorkerStatus::HostConnected).ok();
                                info!("Connected to Host Server!");
                                host_session(&mut state, channels.clone(), session).await;
                            }
                            Err(e) => {
                                error!("Could not connect to host: {}", e);
                                channels.tx_status.send(NetWorkerStatus::NetError(e)).ok();
                            }
                        }
                    }
                    NetWorkerControl::Disconnect => {}
                }
            }
        }
    }
}

async fn host_session(
    wstate: &mut NetWorkerState,
    mut channels: Channels,
    session: HostSessionState,
) {
    loop {
        tokio::select! {
            _ = channels.rx_shutdown.recv() => {
                break;
            }
            e = session.connection.closed() => {
                {
                    error!("Connection closed");
                    // channels.tx_status.send(NetWorkerStatus::NetError(e.into())).ok();
                }
                break;
            }
            Ok(control) = channels.rx_control.recv() => {
                match control {
                    NetWorkerControl::Disconnect | NetWorkerControl::ConnectHost(_) => {
                        break;
                    }
                }
            }
        }
    }
    info!("Disconnected from Host Server!");
    channels
        .tx_status
        .send(NetWorkerStatus::HostDisconnected)
        .ok();
}

pub fn main(config: NetWorkerConfig, channels: Channels) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name_fn(|| "minewars-net-worker".into())
        .build()
        .expect("Cannot create tokio runtime!");

    rt.block_on(async_main(config, channels));
}
