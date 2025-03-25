use bevy::app::AppExit;
use bevy::audio::Volume;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const BASKET_WIDTH: f32 = 128.0;
pub const BANANA_HEIGHT: f32 = 70.0;
pub const BANANA_SPAWN_TIMER_IN_SECONDS: f32 = 0.5;
pub const BANANA_SPEED: f32 = 800.0;

pub const BOUND_SIZE: f32 = 120.0;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.6, 0.7568627451, 0.9450980392);

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
        .init_resource::<FallingObjectSpawnTimer>()
        .init_resource::<Score>()
        .add_systems(
            Startup,
            (
                spawn_background,
                spawn_basket,
                spawn_camera,
                spawn_score_text,
                play_music,
                load_and_cache_images
            )
        )
        .add_systems(
            Update,
            (
                falling_object_movement,
                falling_object_hit_basket,
                falling_object_hit_ground,
                tick_falling_object_spawn_timer,
                spawn_falling_objects_over_time,
                update_basket_position,
                update_score,
                close_on_escape
            )
        )
        .run();
}

#[derive(Component)]
pub struct FallingObject {
    kind: FallingObjectKind,
    points: u32,
}

#[derive(Clone, Copy)]
pub enum FallingObjectKind {
    Banana,
    BananaBunch,
}

#[derive(Component)]
pub struct Basket {}

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct FallingObjectSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct ImageCache {
    banana: Handle<Image>,
    bunch_of_bananas: Handle<Image>,
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

impl Default for FallingObjectSpawnTimer {
    fn default() -> FallingObjectSpawnTimer {
        FallingObjectSpawnTimer {
            timer: Timer::from_seconds(BANANA_SPAWN_TIMER_IN_SECONDS, TimerMode::Repeating),
        }
    }
}

pub fn load_and_cache_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ImageCache {
        banana: asset_server.load("sprites/banana.png"),
        bunch_of_bananas: asset_server.load("sprites/bananabunch.png"),
    });
}

pub fn falling_object_movement(
    mut object_query: Query<&mut Transform, With<FallingObject>>,
    time: Res<Time>,
) {
    for mut transform in object_query.iter_mut() {
        let direction = Vec3::new(0.0, -1.0, 0.0);
        transform.translation += direction * BANANA_SPEED * time.delta_secs();
    }
}

pub fn falling_object_hit_basket(
    mut commands: Commands,
    mut object_query: Query<(Entity, &Transform, &FallingObject)>,
    basket_query: Query<&Transform, With<Basket>>,
    mut score: ResMut<Score>,
) {
    if let Ok(basket_transform) = basket_query.get_single() {
        for (object_entity, object_transform, falling_object) in object_query.iter_mut() {
            let distance = object_transform
                .translation
                .distance(basket_transform.translation);

            let basket_radius = BASKET_WIDTH / 2.0;
            let object_radius = match falling_object.kind {
                FallingObjectKind::Banana => BANANA_HEIGHT / 2.0,
                FallingObjectKind::BananaBunch => BANANA_HEIGHT / 2.0,
            };

            if distance < basket_radius + object_radius {
                score.value += falling_object.points;
                commands.entity(object_entity).despawn();
            }
        }
    }
}

pub fn spawn_falling_objects_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    falling_object_spawn_timer: Res<FallingObjectSpawnTimer>,
    image_cache: Res<ImageCache>,
) {
    if falling_object_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let bounds_width = window.width() - (2.0 * BOUND_SIZE);

        let random_x = (random::<f32>() * bounds_width) + BOUND_SIZE;

        let (object_kind, points, texture) = if random::<f32>() < 0.1 {
            (
                FallingObjectKind::BananaBunch,
                5,
                image_cache.bunch_of_bananas.clone_weak(),
            )
        } else {
            (
                FallingObjectKind::Banana,
                1,
                image_cache.banana.clone_weak(),
            )
        };

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_xyz(random_x, window.height(), 0.0),
            FallingObject {
                kind: object_kind,
                points,
            },
        ));
    }
}

pub fn falling_object_hit_ground(
    mut commands: Commands,
    mut falling_object_query: Query<(Entity, &Transform), With<FallingObject>>,
) {
    let ground_y = Vec3::new(0.0, 0.0, 0.0);

    for (entity, transform) in falling_object_query.iter_mut() {
        let entity_y = Vec3::new(0.0, transform.translation.y, 0.0);

        let distance = ground_y.distance(entity_y);

        if distance < 5.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("audio/main_theme.ogg")), 
        PlaybackSettings::LOOP.with_volume(Volume::new(1.0))
    ));

}

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/hot_air_balloon.png")),
        Transform::from_xyz(window.width() / 6.0, window.height() / 1.25, -1.0)
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/tree.png")),
        Transform::from_xyz(window.width() / 1.5, window.height() / 4.5, -1.0)
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/cloud_small.png")),
        Transform::from_xyz(window.width() / 2.5, window.height() / 1.75, -1.0)
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/cloud_big.png")),
        Transform::from_xyz(window.width() / 1.1, window.height(), -1.0)
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ground.png")),
        Transform::from_xyz(window.width() / 2.0, 119.0 / 2.0, -2.0)
    ));
}

pub fn spawn_basket(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/basket.png")),
        Transform::from_xyz(window.width() / 2.0, BOUND_SIZE, 0.0),
        Basket {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
    ));
}

pub fn spawn_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 60.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        TextColor::BLACK
    ))
    .with_child((
        TextSpan::default(),
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 60.0,
            ..default()
        },
        TextColor::BLACK,
        ScoreText
    ));
}

pub fn tick_falling_object_spawn_timer(
    mut falling_object_spawn_timer: ResMut<FallingObjectSpawnTimer>,
    time: Res<Time>,
) {
    falling_object_spawn_timer.timer.tick(time.delta());
}

pub fn update_basket_position(
    mut events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut basket_query: Query<&mut Transform, With<Basket>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut transform) = basket_query.get_single_mut() {
        for event in events.read() {
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

pub fn update_score(
    mut score_text_query: Query<&mut TextSpan, With<ScoreText>>, 
    score: Res<Score>
) {
    if score.is_changed() {
        for mut text in &mut score_text_query {
            **text = format!("{0}", score.value);
        }
    }
}

fn close_on_escape(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Escape && event.state.is_pressed() {
            app_exit_events.send(AppExit::Success);
        }
    }
}
