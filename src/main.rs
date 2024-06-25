use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::Velocity;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::Movable;
use player::PlayerPlugin;

mod components;
mod player;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const SCREEN_SIZE_X: f32 = 900.;
const SCREEN_SIZE_Y: f32 = 1200.;
const PLAYER_SPRITE: &str = "player_icon_turtle_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_SIZE: (f32, f32) = (92., 92.);
// const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const SPRITE_SCALE: f32 = 0.25;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders".into(),
                resolution: (SCREEN_SIZE_X, SCREEN_SIZE_Y).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_system)
        .add_systems(Update, movable_system)
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // Window size
    let Ok(primary) = query.get_single() else {
        return;
    };
    let (win_w, win_h) = (primary.width(), primary.height());
    println!("widht: {win_w}, height: {win_h}");

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);

    let rectangle_outer = Mesh2dHandle(meshes.add(Rectangle::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)));
    let rectangle_inner =
        Mesh2dHandle(meshes.add(Rectangle::new(SCREEN_SIZE_X - 10., SCREEN_SIZE_Y - 10.)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle_outer,
        material: materials.add(Color::rgb_u8(255, 255, 255)),
        transform: Transform::from_xyz(0.0, 0.0, -21.), // Slightly forward in Z
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle_inner,
        material: materials.add(Color::rgb_u8(0, 0, 0)),
        transform: Transform::from_xyz(0.0, 0.0, -20.), // Even more forward in Z
        ..default()
    });
}

fn movable_system(
    mut commands: Commands,
    winsize: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable, &Name)>,
) {
    for (entity, velocity, mut transform, movable, name) in query.iter_mut() {
        transform.translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        transform.translation.y += velocity.y * BASE_SPEED * TIME_STEP;

        if name.as_str() == "player" {
            transform.translation.x = transform.translation.x.clamp(
                -winsize.w / 2. + (PLAYER_SIZE.0 / 2.),
                winsize.w / 2. - (PLAYER_SIZE.0 / 2.),
            );
            transform.translation.y = transform.translation.y.clamp(
                -winsize.h / 2. + (PLAYER_SIZE.1 / 2.),
                winsize.h / 2. - (PLAYER_SIZE.1 / 2.),
            );
        }
        if movable.auto_despawn {
            if transform.translation.y > winsize.h / 2. {
                commands.entity(entity).despawn();
            }
        }
    }
}
