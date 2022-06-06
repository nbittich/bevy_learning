use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const GAME_WIDTH: f32 = 1600.;
pub const GAME_HEIGHT: f32 = 900.;
pub const WALL_WIDTH: f32 = 80.;
pub const WALL_HEIGHT: f32 = 80.;

pub const BASE_SPEED: f32 = 400.;
pub const TIME_STEP: f32 = 1. / 60.;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component, Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}

pub struct GlobalAssets {
    pub spaceship: Handle<Image>,
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
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_system(player_movement_system)
        .add_system(movement_system)
        .add_system(spaw_player_projectile)
        .run();
}

fn setup_system(mut commands: Commands, assets_server: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.insert_resource(GlobalAssets {
        spaceship: assets_server.load("spaceship.png"),
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
        if check_x < -GAME_WIDTH / 2. + WALL_WIDTH ||check_x > GAME_WIDTH / 2. - WALL_WIDTH {
            x = 0.;
        }

        if check_y < -GAME_HEIGHT / 2. + WALL_HEIGHT || check_y > GAME_HEIGHT / 2. - WALL_HEIGHT {
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
                    translation: transform_player.translation.clone(),
                    scale: Vec3::new(0.05, 0.05, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerProjectile)
            .insert(Velocity { x: 0., y: 1. });
    }
}

