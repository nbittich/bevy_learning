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

#[derive(Component, Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}

pub struct GlobalAssets {
    pub spaceship: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
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
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_system(player_movement_system)
        .add_system(movement_system)
        .run();
}

fn setup_system(mut commands: Commands, assets_server: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.insert_resource(GlobalAssets {
        spaceship: assets_server.load("spaceship.png"),
    });

    let background: Handle<Image> = assets_server.load("space.svg");
    commands.spawn_bundle(SpriteBundle{
        texture: background,
        transform: Transform {
            translation: Vec3::new(GAME_WIDTH, GAME_HEIGHT, 30.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
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
        let (mut x, mut y) = (transform.translation.x, transform.translation.y);
        x += velocity.x * BASE_SPEED * TIME_STEP;
        y += velocity.y * BASE_SPEED * TIME_STEP;

        if x > -GAME_WIDTH / 2. + WALL_WIDTH && x < GAME_WIDTH / 2. - WALL_WIDTH {
            transform.translation.x = x;
        }

        if y > -GAME_HEIGHT / 2. + WALL_HEIGHT && y < GAME_HEIGHT / 2. - WALL_HEIGHT {
            transform.translation.y = y;
        }
    }
}

fn player_movement_system(
    mut query: Query<&mut Velocity, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        if !input.pressed(KeyCode::Right)
            && !input.pressed(KeyCode::Left)
            && !input.pressed(KeyCode::Up)
            && !input.pressed(KeyCode::Down)
        {
            velocity.x = 0.;
            velocity.y = 0.;
        } else {
            if input.pressed(KeyCode::Right) {
                velocity.x = 1.;
            }
            if input.pressed(KeyCode::Left) {
                velocity.x = -1.;
            }
            if input.pressed(KeyCode::Up) {
                velocity.y = 1.;
            }
            if input.pressed(KeyCode::Down) {
                velocity.y = -1.;
            }
        }
    }
}
