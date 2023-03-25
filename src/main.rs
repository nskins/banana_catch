use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const BASKET_WIDTH: f32 = 128.0;
pub const BANANA_HEIGHT: f32 = 70.0;
// When you set the `BANANA_SPAWN_TIMER_IN_SECONDS` to 1.0, a bug
// occurs when you catch a banana at the top of the basket. The
// other banana that has "just spawned" will disappear until the
// next banana spawns. Should look into this more.
pub const BANANA_SPAWN_TIMER_IN_SECONDS: f32 = 0.5;
pub const BANANA_SPEED: f32 = 800.0;

pub const BOUND_SIZE: f32 = 120.0;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.6, 0.7568627451, 0.9450980392);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600., 1000.).into(),
                resizable: false,
                title: "Banana Catch".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<BananaSpawnTimer>()
        .init_resource::<Score>()
        .add_startup_system(spawn_background)
        .add_startup_system(spawn_basket)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_score_text)
        .add_startup_system(play_music)
        .add_system(banana_movement)
        .add_system(banana_hit_basket)
        .add_system(banana_hit_ground)
        .add_system(tick_banana_spawn_timer)
        .add_system(spawn_bananas_over_time)
        .add_system(update_basket_position)
        .add_system(update_score)
        .add_system(close_on_escape)
        .run();
}

#[derive(Component)]
pub struct Banana {}

#[derive(Component)]
pub struct Basket {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Resource)]
pub struct BananaSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

impl Default for BananaSpawnTimer {
    fn default() -> BananaSpawnTimer {
        BananaSpawnTimer {
            timer: Timer::from_seconds(BANANA_SPAWN_TIMER_IN_SECONDS, TimerMode::Repeating),
        }
    }
}

pub fn banana_movement(mut banana_query: Query<(&mut Transform, &Banana)>, time: Res<Time>) {
    for (mut transform, _banana) in banana_query.iter_mut() {
        let direction = Vec3::new(0.0, -1.0, 0.0);

        transform.translation += direction * BANANA_SPEED * time.delta_seconds();
    }
}

pub fn banana_hit_basket(
    mut commands: Commands,
    mut banana_query: Query<(Entity, &Transform), With<Banana>>,
    basket_query: Query<&Transform, With<Basket>>,
    mut score: ResMut<Score>,
) {
    if let Ok(basket_transform) = basket_query.get_single() {
        for (banana_entity, banana_transform) in banana_query.iter_mut() {
            let distance = banana_transform
                .translation
                .distance(basket_transform.translation);

            let basket_radius = BASKET_WIDTH / 2.0;
            let banana_radius = BANANA_HEIGHT / 2.0;

            if distance < basket_radius + banana_radius {
                score.value += 1;
                commands.entity(banana_entity).despawn();
            }
        }
    }
}

pub fn banana_hit_ground(
    mut commands: Commands,
    mut banana_query: Query<(Entity, &Transform), With<Banana>>,
) {
    let ground_y = Vec3::new(0.0, 0.0, 0.0);

    for (entity, transform) in banana_query.iter_mut() {
        let entity_y = Vec3::new(0.0, transform.translation.y, 0.0);

        let distance = ground_y.distance(entity_y);

        if distance < 5.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_with_settings(
        asset_server.load("audio/main_theme.ogg"),
        PlaybackSettings::LOOP.with_volume(1.0),
    );
}

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 6.0, window.height() / 1.25, -1.0),
        texture: asset_server.load("sprites/hot_air_balloon.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 1.5, window.height() / 4.5, -1.0),
        texture: asset_server.load("sprites/tree.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.5, window.height() / 1.75, -1.0),
        texture: asset_server.load("sprites/cloud_small.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 1.1, window.height(), -1.0),
        texture: asset_server.load("sprites/cloud_big.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, 119.0 / 2.0, -2.0),
        texture: asset_server.load("sprites/ground.png"),
        ..default()
    });
}

pub fn spawn_basket(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, BOUND_SIZE, 0.0),
            texture: asset_server.load("sprites/basket.png"),
            ..default()
        },
        Basket {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_score_text(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                format!("Score: {0}", score.value),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText {},
    ));
}

pub fn tick_banana_spawn_timer(mut banana_spawn_timer: ResMut<BananaSpawnTimer>, time: Res<Time>) {
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

        let bounds_width = window.width() - (2.0 * BOUND_SIZE);

        let random_x = (random::<f32>() * bounds_width) + BOUND_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, window.height(), 0.0),
                texture: asset_server.load("sprites/banana.png"),
                ..default()
            },
            Banana {},
        ));
    }
}

pub fn update_basket_position(
    mut events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut basket_query: Query<&mut Transform, With<Basket>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut transform) = basket_query.get_single_mut() {
        for event in events.iter() {
            if event.position.x < BOUND_SIZE {
                transform.translation.x = BOUND_SIZE;
            } else if event.position.x > window.width() - BOUND_SIZE {
                transform.translation.x = window.width() - BOUND_SIZE;
            } else {
                transform.translation.x = event.position.x;
            }
        }
    }
}

pub fn update_score(mut score_text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = score_text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {0}", score.value);
        }
    }
}

fn close_on_escape(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.iter() {
        if event.key_code == Some(KeyCode::Escape) && event.state.is_pressed() {
            app_exit_events.send(AppExit);
        }
    }
}
