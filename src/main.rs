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
        .add_startup_system(play_music)
        .add_system(update_basket_position)
        .run();
}

#[derive(Component)]
pub struct Basket {}

pub fn play_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    audio.play_with_settings(
        asset_server.load("audio/main_theme.ogg"),
        PlaybackSettings::LOOP.with_volume(1.0)
    );
}

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

pub fn update_basket_position(
    mut events: EventReader<CursorMoved>,
    mut basket_query: Query<&mut Transform, With<Basket>>,
) {
    if let Ok(mut transform) = basket_query.get_single_mut() {
        for event in events.iter() {
            transform.translation.x = event.position.x;
        }
    }
}

