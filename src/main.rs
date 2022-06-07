use std::time::Duration;

use bevy::{
    audio::AudioSink,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const GAME_WIDTH: f32 = 1600.;
pub const GAME_HEIGHT: f32 = 900.;
pub const WALL_WIDTH: f32 = 80.;
pub const WALL_HEIGHT: f32 = 80.;

pub const BASE_SPEED: f32 = 400.;
pub const TIME_STEP: f32 = 1. / 60.;

pub const MAX_ENNEMIES: usize = 10;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ennemy {
    can_follow: bool,
}
#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component)]
pub struct EnnemyProjectile;
#[derive(Component)]
pub struct ShootProjectile {
    timer: Timer,
}

#[derive(Component, Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}
#[derive(Default)]
pub struct SpawnEnnemyConfig {
    timer: Timer,
}

pub struct GlobalAssets {
    pub spaceship: Handle<Image>,
    pub alien: Handle<Image>,
    pub projectile1: Handle<Image>,
    pub projectile2: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
    pub explosion_len: usize,
    pub font: Handle<Font>,
}
#[derive(Component)]
pub struct Health(usize);
#[derive(Component)]
pub struct EnnemyHealth(usize);
#[derive(Component)]
pub struct Score(usize);
#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct HealthText;
#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    index: usize,
    audio: Handle<AudioSink>,
}

fn main() {
    App::new()
        // .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            title: "Krusty".to_string(),
            resizable: true,
            present_mode: bevy::window::PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_system(player_movement_system)
        .add_system(movement_system)
        .add_system(spaw_player_projectile)
        .add_system(spawn_ennemy)
        .add_system(ennemy_movement)
        .add_system(spaw_ennemy_projectile)
        .add_system(update_health)
        .add_system(update_health_text)
        .add_system(update_score)
        .add_system(update_score_text)
        .add_system(handle_explosion)
        .add_system(despawn_out_screen)
        .insert_resource(SpawnEnnemyConfig {
            timer: Timer::from_seconds(2., true),
        })
        .run();
}

fn setup_system(
    mut commands: Commands,
    audio: Res<Audio>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets_server: Res<AssetServer>,
) {
    let camera = OrthographicCameraBundle::new_2d();
    let camera_ui = UiCameraBundle::default();

    commands.spawn_bundle(camera);
    commands.spawn_bundle(camera_ui);

    let texture_atlas = TextureAtlas::from_grid(
        assets_server.load("explosionframes.png"),
        Vec2::splat(128.),
        8,
        3,
    );

    let explosion_len = texture_atlas.len();

    commands.insert_resource(GlobalAssets {
        spaceship: assets_server.load("spaceship.png"),
        alien: assets_server.load("alien.png"),
        projectile1: assets_server.load("projectile1.png"),
        projectile2: assets_server.load("projectile2.png"),
        font: assets_server.load("KdamThmorPro-Regular.ttf"),
        explosion: texture_atlases.add(texture_atlas),
        explosion_len,
    });

    let music = assets_server.load("ryu.mp3");

    audio.play(music);

    let background: Handle<Image> = assets_server.load("space.png");
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.scale = Vec3::new(1.8, 1.8, 1.0);
    commands.spawn_bundle(SpriteBundle {
        texture: background,
        transform,
        ..Default::default()
    });
}

fn spawn_player_system(mut commands: Commands, assets: Res<GlobalAssets>) {
    let score = 0;
    let health = 100;

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.spaceship.clone(),
            transform: Transform {
                translation: Vec3::new(0., -GAME_HEIGHT / 2. + WALL_HEIGHT, 10.),
                scale: Vec3::new(1., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::default())
        .insert(Health(health))
        .insert(Score(score));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(50.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                format!("Score: {score}"),
                TextStyle {
                    font_size: 60.0,
                    font: assets.font.clone(),
                    color: Color::AQUAMARINE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..default()
                },
            ),

            ..Default::default()
        })
        .insert(ScoreText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                format!("Health: {health}"),
                TextStyle {
                    font_size: 60.0,
                    font: assets.font.clone(),
                    color: Color::AQUAMARINE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment { ..default() },
            ),

            ..Default::default()
        })
        .insert(HealthText);
}

fn movement_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        transform.translation.y += velocity.y * BASE_SPEED * TIME_STEP;
    }
}

fn player_movement_system(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((mut velocity, transform)) = query.get_single_mut() {
        let (mut x, mut y) = (velocity.x, velocity.y);
        if !input.pressed(KeyCode::Right)
            && !input.pressed(KeyCode::Left)
            && !input.pressed(KeyCode::Up)
            && !input.pressed(KeyCode::Down)
        {
            x = 0.;
            y = 0.;
        } else {
            if input.pressed(KeyCode::Right) {
                x = 1.;
            }
            if input.pressed(KeyCode::Left) {
                x = -1.;
            }
            if input.pressed(KeyCode::Up) {
                y = 1.;
            }
            if input.pressed(KeyCode::Down) {
                y = -1.;
            }
        }

        let check_x = transform.translation.x + x;
        let check_y = transform.translation.y + y;

        // todo this is dumb
        if !(-GAME_WIDTH / 2. + WALL_WIDTH..=GAME_WIDTH / 2. - WALL_WIDTH).contains(&check_x) {
            x = 0.;
        }

        if !(-GAME_HEIGHT / 2. + WALL_HEIGHT..=GAME_HEIGHT / 2. - WALL_HEIGHT).contains(&check_y) {
            y = 0.;
        }

        velocity.x = x;
        velocity.y = y;
    }
}

fn spaw_player_projectile(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    audio: Res<Audio>,
    assets_server: Res<AssetServer>,
    query_player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let transform_player = query_player.single();
        commands
            .spawn_bundle(SpriteBundle {
                texture: global_assets.projectile2.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        transform_player.translation.x,
                        transform_player.translation.y,
                        1.,
                    ),
                    scale: Vec3::new(0.05, 0.05, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerProjectile)
            .insert(Velocity { x: 0., y: 1. });
        let music = assets_server.load("laser_sound.wav");

        audio.play(music);
    }
}

fn spawn_ennemy(
    mut commands: Commands,
    assets: Res<GlobalAssets>,
    time: Res<Time>,
    mut config: ResMut<SpawnEnnemyConfig>,
    query_ennemies: Query<&Ennemy>,
) {
    config.timer.tick(time.delta());
    let ennemies_already_spawned: usize = query_ennemies.iter().count();
    if config.timer.finished() && ennemies_already_spawned < MAX_ENNEMIES {
        let rand_w = rand::thread_rng()
            .gen_range(-GAME_WIDTH / 2. + WALL_HEIGHT..GAME_WIDTH / 2. - WALL_HEIGHT);
        commands
            .spawn_bundle(SpriteBundle {
                texture: assets.alien.clone(),
                transform: Transform {
                    translation: Vec3::new(rand_w, GAME_HEIGHT / 2. - WALL_HEIGHT, 10.),
                    scale: Vec3::new(0.5, 0.5, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Ennemy {
                can_follow: rand::thread_rng().gen_bool(1. / 5.),
            })
            .insert(Velocity { x: 0., y: -0.2 })
            .insert(ShootProjectile {
                timer: Timer::new(Duration::from_millis(80 * 6), true),
            })
            .insert(EnnemyHealth(rand::thread_rng().gen_range(2..5)));
    }
}

fn despawn_out_screen(mut commands: Commands, query_velocity: Query<(Entity, &Transform)>) {
    for (ent, transform) in query_velocity.iter() {
        if transform.translation.y < -GAME_HEIGHT || transform.translation.y > GAME_HEIGHT {
            commands.entity(ent).despawn_recursive();
        }
    }
}
fn ennemy_movement(
    query_player: Query<&Transform, With<Player>>,
    mut query_ennemy: Query<(&Ennemy, &mut Velocity, &Transform)>,
) {
    let player_transform = query_player.single();
    for (ennemy, mut velocity, transform) in query_ennemy.iter_mut() {
        let (x, _y) = (transform.translation.x, transform.translation.y);
        if ennemy.can_follow {
            let delta = (x.round() - player_transform.translation.x.round()).abs();
            if delta > 5. {
                if x > player_transform.translation.x {
                    velocity.x -= 0.3;
                } else if x < player_transform.translation.x {
                    velocity.x = 1.;
                }
            } else {
                velocity.x = 0.;
            }
        }
    }
}

fn spaw_ennemy_projectile(
    time: Res<Time>,
    audio: Res<Audio>,
    assets_server: Res<AssetServer>,
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    mut query_ennemy: Query<(&mut ShootProjectile, &Transform), With<Ennemy>>,
) {
    let delta = time.delta();

    for (mut shoot_projectile, transform_ennemy) in query_ennemy.iter_mut() {
        shoot_projectile.timer.tick(delta);
        if shoot_projectile.timer.finished() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: global_assets.projectile1.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            transform_ennemy.translation.x,
                            transform_ennemy.translation.y,
                            1.,
                        ),
                        scale: Vec3::new(0.05, 0.05, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(EnnemyProjectile)
                .insert(Velocity { x: 0., y: -1. });

            let music = assets_server.load("laserfire02.ogg");

            audio.play(music);
        }
    }
}

fn update_health(
    mut commands: Commands,
    mut query_player: Query<(&mut Health, &Transform), With<Player>>,
    query_ennemy_projectiles: Query<(Entity, &Transform), With<EnnemyProjectile>>,
) {
    let (mut health, player_transform) = query_player.single_mut();

    for (proj_ent, proj_transform) in query_ennemy_projectiles.iter() {
        let collision = collide(
            player_transform.translation,
            Vec2::splat(120.),
            proj_transform.translation,
            Vec2::splat(12.8),
        );

        if let Some(Collision::Inside) = collision {
            health.0 -= 1;
            commands.entity(proj_ent).despawn_recursive();
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn update_score(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    assets_server: Res<AssetServer>,
    audio_sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
    mut query_player: Query<&mut Score, With<Player>>,
    mut query_ennemies: Query<(Entity, &mut EnnemyHealth, &Transform), With<Ennemy>>,
    query_projectiles: Query<(Entity, &Transform), With<PlayerProjectile>>,
) {
    let mut score = query_player.single_mut();
    for (ennemy_ent, mut health, ennemy_transform) in query_ennemies.iter_mut() {
        for (proj_ent, proj_transform) in query_projectiles.iter() {
            let collision = collide(
                ennemy_transform.translation,
                Vec2::splat(120.),
                proj_transform.translation,
                Vec2::splat(12.8),
            );

            if let Some(Collision::Inside) = collision {
                health.0 -= 1;
                commands.entity(proj_ent).despawn();
                if health.0 == 0 {
                    let music = assets_server.load("explosion.wav");
                    let handle = audio_sinks.get_handle(audio.play(music));
                    let explosion = global_assets.explosion.clone();
                    let _explosion_id = commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: explosion,
                            transform: *ennemy_transform,
                            ..Default::default()
                        })
                        .insert(Explosion {
                            timer: Timer::from_seconds(0.1, false),
                            index: 0,
                            audio: handle,
                        })
                        .id();

                    // commands.entity(explosion_id).despawn_recursive();
                    commands.entity(ennemy_ent).despawn_recursive();
                    score.0 += 1;
                }
            }
        }
    }
}

fn handle_explosion(
    mut commands: Commands,
    time: Res<Time>,
    audio_sinks: Res<Assets<AudioSink>>,
    global_assets: Res<GlobalAssets>,
    mut query_explosion: Query<(Entity, &mut Explosion, &mut TextureAtlasSprite)>,
) {
    let delta = time.delta();

    for (ent, mut expl, mut texture) in query_explosion.iter_mut() {
        expl.timer.tick(delta);

        if expl.timer.finished() {
            if let Some(sink) = audio_sinks.get(&expl.audio) {
                if expl.index == 0 {
                    sink.play();
                } else if global_assets.explosion_len <= expl.index {
                    sink.stop();
                }
            }
            if global_assets.explosion_len > expl.index {
                texture.index = expl.index;
                expl.index += 1;
            } else {
                commands.entity(ent).despawn();
            }
        }
    }
}

fn update_health_text(
    query_player: Query<&Health, With<Player>>,
    mut query: Query<&mut Text, With<HealthText>>,
) {
    let health = query_player.single();
    let mut health_text = query.single_mut();
    health_text.sections[0].value = format!("Health: {}", health.0);
}
fn update_score_text(
    query_player: Query<&Score, With<Player>>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    let score = query_player.single();
    let mut score_text = query.single_mut();
    score_text.sections[0].value = format!("Score: {}", score.0);
}
