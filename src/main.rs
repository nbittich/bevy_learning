use bevy::prelude::*;
use rand::Rng;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const GAME_WIDTH: f32 = 1600.;
pub const GAME_HEIGHT: f32 = 900.;
pub const WALL_WIDTH: f32 = 80.;
pub const WALL_HEIGHT: f32 = 80.;

pub const BASE_SPEED: f32 = 400.;
pub const TIME_STEP: f32 = 1. / 60.;

pub const MAX_ENNEMIES: usize = 3;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ennemy;
#[derive(Component)]
pub struct PlayerProjectile;

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
    pub projectile2: Handle<Image>,
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
        .add_system(despawn_out_screen)
        .insert_resource(SpawnEnnemyConfig {
            timer: Timer::from_seconds(2., true),
        })
        .run();
}

fn setup_system(mut commands: Commands, assets_server: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.insert_resource(GlobalAssets {
        spaceship: assets_server.load("spaceship.png"),
        alien: assets_server.load("alien.png"),
        projectile2: assets_server.load("projectile2.png"),
    });
    commands.spawn_bundle(camera);

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
        .insert(Velocity::default());
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
    query_player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let transform_player = query_player.single();
        commands
            .spawn_bundle(SpriteBundle {
                texture: global_assets.projectile2.clone(),
                transform: Transform {
                    translation: transform_player.translation,
                    scale: Vec3::new(0.05, 0.05, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerProjectile)
            .insert(Velocity { x: 0., y: 1. });
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
                    translation: Vec3::new(rand_w , GAME_HEIGHT, 10.),
                    scale: Vec3::new(0.5, 0.5, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Ennemy)
            .insert(Velocity { x: 0., y: -0.3 });
    }
}

fn despawn_out_screen(mut commands: Commands, query_velocity: Query<(Entity, &Transform)>) {
    for (ent, transform) in query_velocity.iter() {
        if transform.translation.y < -GAME_HEIGHT || transform.translation.y > GAME_HEIGHT {
            commands.entity(ent).despawn();
        }
    }
}
fn ennemy_movement(
    query_player: Query<&Transform, With<Player>>,
    mut query_ennemy: Query<(&mut Velocity, &Transform), With<Ennemy>>,
) {
    let player_transform = query_player.single();
    for (mut velocity, transform) in query_ennemy.iter_mut() {
        if transform.translation.x.ceil() > player_transform.translation.x.ceil() {
            velocity.x = -1.;
        } else if transform.translation.x.ceil() < player_transform.translation.x.ceil() {
            velocity.x = 1.;
        } else {
            velocity.x = 0.;
        }
        //  velocity.y = -rand::thread_rng().gen_range(0. .. 1.);
    }
}
