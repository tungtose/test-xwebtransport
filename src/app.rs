use bevy::prelude::*;

// use crate::{net::NetClientPlugin, ui::UiPlugin};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(NetClientPlugin)
        // .add_plugins(UiPlugin)
        .run();
}
