use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600., 1000.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_basket)
        .add_startup_system(spawn_camera)
        .run();
}

#[derive(Component)]
pub struct Basket {}

pub fn spawn_basket(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 120.0, 0.0),
            texture: asset_server.load("sprites/bucket.png"),
            ..default()
        },
        Basket {},
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

