#![windows_subsystem = "windows"]

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "urban-telegram".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .run();
}
