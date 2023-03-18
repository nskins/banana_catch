use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const BANANA_SPAWN_TIMER_IN_SECONDS: f32 = 3.0;
pub const BANANA_SPEED: f32 = 800.0;

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
        .init_resource::<BananaSpawnTimer>()
        .add_startup_system(spawn_basket)
        .add_startup_system(spawn_camera)
        .add_startup_system(play_music)
        .add_system(banana_movement)
        .add_system(tick_banana_spawn_timer)
        .add_system(spawn_bananas_over_time)
        .add_system(update_basket_position)
        .run();
}

#[derive(Component)]
pub struct Banana {}

#[derive(Component)]
pub struct Basket {}

#[derive(Resource)]
pub struct BananaSpawnTimer {
    pub timer: Timer,
}

impl Default for BananaSpawnTimer {
    fn default() -> BananaSpawnTimer {
        BananaSpawnTimer {
            timer: Timer::from_seconds(BANANA_SPAWN_TIMER_IN_SECONDS, TimerMode::Repeating),
        }
    }
}

pub fn banana_movement(
    mut banana_query: Query<(&mut Transform, &Banana)>,
    time: Res<Time>,
) {
    for (mut transform, _banana) in banana_query.iter_mut() {
        let direction = Vec3::new(0.0, -1.0, 0.0);

        transform.translation += direction * BANANA_SPEED * time.delta_seconds();
    }
}

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

pub fn tick_banana_spawn_timer(
    mut banana_spawn_timer: ResMut<BananaSpawnTimer>,
    time: Res<Time>
) {
    banana_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_bananas_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    banana_spawn_timer: Res<BananaSpawnTimer>,
) {
    if banana_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();

        commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, window.height(), 0.0),
                    texture: asset_server.load("sprites/banana.png"),
                    ..default()
                },
                Banana {}
        ));
    }
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

