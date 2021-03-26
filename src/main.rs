use bevy::prelude::{App, WindowDescriptor};
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1920.,
            height: 1080.,
            title: "urban-telegram".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
