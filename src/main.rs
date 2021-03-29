#![windows_subsystem = "windows"]

use bevy::prelude::{App, WindowDescriptor};
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "urban-telegram".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
