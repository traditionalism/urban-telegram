#![windows_subsystem = "windows"]

use bevy::{pbr::AmbientLight, input::system::exit_on_esc_system, prelude::*};
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "urban-telegram".to_string(),
            width: 1600.,
            height: 1600.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 2.0;
    camera.transform = Transform::from_xyz(1.0, 1.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn_scene(asset_server.load("models/sheen/SheenChair.gltf#Scene0"));
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}