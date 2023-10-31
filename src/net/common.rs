pub mod prelude {
    pub use super::{RxShutdown, TxShutdown};
    pub use tokio::sync::broadcast::{Receiver as RxBroadcast, Sender as TxBroadcast};
    pub use tokio::sync::mpsc::{
        Receiver as RxMpsc, Sender as TxMpsc, UnboundedReceiver as RxMpscU,
        UnboundedSender as TxMpscU,
    };
    pub use tokio::sync::oneshot::{Receiver as RxOneshot, Sender as TxOneshot};
}

// use core::hash::Hash;
// use std::{path::Path, sync::Arc};

// use bevy::utils::HashSet;
pub use prelude::*;

// use tokio;

// use rustls::{server::AllowAnyAuthenticatedClient, Certificate, PrivateKey, RootCertStore};
use serde::{Deserialize, Serialize};
use wtransport::{endpoint::endpoint_side, Endpoint};

pub type TxShutdown = TxBroadcast<()>;
pub type RxShutdown = RxBroadcast<()>;

/// How to interpret a list of restrictions for security
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlListMode {
    /// Everything else, except for what is in the list, is allowed
    Denylist,
    /// Only what is in the list is allowed
    Allowlist,
}

// pub async fn load_cert(path: &Path) -> Result<Certificate, tokio::io::Error> {
//     let bytes = tokio::fs::read(path).await?;
//     Ok(Certificate(bytes))
// }

// pub async fn load_key(path: &Path) -> Result<PrivateKey, tokio::io::Error> {
//     let bytes = tokio::fs::read(path).await?;
//     Ok(PrivateKey(bytes))
// }

// pub fn check_list<T: Eq + Hash>(mode: ControlListMode, list: &HashSet<T>, value: &T) -> bool {
//     match mode {
//         ControlListMode::Denylist => !list.contains(value),
//         ControlListMode::Allowlist => list.contains(value),
//     }
// }

use anyhow::Result;

// pub async fn load_server_crypto(
//     my_certs: &[impl AsRef<Path>],
//     my_key: impl AsRef<Path>,
//     client_verification: bool,
//     client_ca: impl AsRef<Path>,
// ) -> Result<Arc<rustls::ServerConfig>> {
//     let mut my_certs_data = vec![];
//     for cert in my_certs {
//         my_certs_data.push(load_cert(cert.as_ref()).await?);
//     }
//     let my_key_data = load_key(my_key.as_ref()).await?;
//     let client_ca_data = if client_verification {
//         Some(load_cert(client_ca.as_ref()).await?)
//     } else {
//         None
//     };
//     // setup_server_crypto(&my_certs_data, &my_key_data, client_ca_data.as_ref())
// }

// pub async fn load_client_crypto(
//     server_ca: impl AsRef<Path>,
//     client_verification: bool,
//     my_certs: &[impl AsRef<Path>],
//     my_key: impl AsRef<Path>,
// ) -> Result<Arc<rustls::ClientConfig>> {
//     let server_ca_data = load_cert(server_ca.as_ref()).await?;
//     if client_verification {
//         let mut my_certs_data = vec![];
//         for cert in my_certs {
//             my_certs_data.push(load_cert(cert.as_ref()).await?);
//         }
//         let my_key_data = load_key(my_key.as_ref()).await?;
//         setup_client_crypto(Some((&my_certs_data, &my_key_data)), &server_ca_data)
//     } else {
//         setup_client_crypto(None, &server_ca_data)
//     }
// }

// pub fn setup_server_crypto(
//     my_certs: &[Certificate],
//     my_key: &PrivateKey,
//     client_ca: Option<&Certificate>,
// ) -> AnyResult<Arc<rustls::ServerConfig>> {
//     let crypto = rustls::ServerConfig::builder().with_safe_defaults();

//     let crypto = match client_ca {
//         Some(client_ca) => {
//             let mut roots = RootCertStore::empty();
//             roots.add(client_ca)?;
//             crypto.with_client_cert_verifier(AllowAnyAuthenticatedClient::new(roots))
//         }
//         None => crypto.with_no_client_auth(),
//     };

//     let crypto =
//         crypto.with_single_cert(my_certs.into_iter().cloned().collect(), my_key.clone())?;

//     Ok(Arc::new(crypto))
// }

// pub fn setup_client_crypto(
//     my_certs_key: Option<(&[Certificate], &PrivateKey)>,
//     server_ca: &Certificate,
// ) -> Result<Arc<rustls::ClientConfig>> {
//     let crypto = rustls::ClientConfig::builder().with_safe_defaults();

//     let mut roots = RootCertStore::empty();
//     roots.add(server_ca)?;
//     let crypto = crypto.with_root_certificates(roots);

//     let crypto = match my_certs_key {
//         Some((my_certs, my_key)) => {
//             crypto.with_single_cert(my_certs.into_iter().cloned().collect(), my_key.clone())?
//         }
//         None => crypto.with_no_client_auth(),
//     };

//     Ok(Arc::new(crypto))
// }

// pub fn setup_quic_server(
//     crypto: Arc<rustls::ServerConfig>,
//     my_addr: SocketAddr,
// ) -> Result<Endpoint> {
//     let config = quinn::ServerConfig::with_crypto(crypto);
//     let endpoint = Endpoint::server(config, my_addr)?;
//     Ok(endpoint)
// }

pub fn setup_wtransport_client(
    // crypto: Arc<rustls::ClientConfig>,
    addr: &str,
) -> Result<Endpoint<endpoint_side::Client>> {
    let config = wtransport::ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation()
        // .with_custom_tls()
        // .with_custom_tls(*crypto)
        // .with_native_certs()
        .build();

    let mut endpoint = Endpoint::client(config).unwrap();
    // endpoint.set_default_client_config(config);
    Ok(endpoint)
}
