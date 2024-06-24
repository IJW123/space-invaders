use crate::{components::{Player, Velocity}, GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};
use bevy::{ecs::query, input::{keyboard::Key, InputPlugin}, prelude::*};

const TIME_STEP:  f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const MIN_X:      f32 = -450. + (PLAYER_SIZE.0 / 2.);
const MAX_X:      f32 = 450. - (PLAYER_SIZE.0 / 2.);
const MIN_Y:      f32 = -600. + (PLAYER_SIZE.1 / 2.);
const MAX_Y:      f32 = 600. - (PLAYER_SIZE.1 / 2.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, player_spawn_system)
            .add_systems(Update, player_movement_system)
            .add_systems(Update, player_keyboard_event_system)
            .add_systems(Update, player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.;
    commands
        .spawn(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0.0,bottom + PLAYER_SIZE.1 / 2. + 5. , 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity {x: 0., y: 0.});
}

fn player_fire_system (
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>, 
){
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            commands.spawn(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::KeyA) {
            -1.
        } else if kb.pressed(KeyCode::KeyD) {
            1.
        } else {
            0.
        };
        velocity.y = if kb.pressed(KeyCode::KeyW) {
            1.
        } else if kb.pressed(KeyCode::KeyS) {
            -1.
        } else {
            0.
        };    
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>
) {
    for (velocity, mut transform) in query.iter_mut() {
        // let translation = &mut transform.translation;
        transform.translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        transform.translation.y += velocity.y * BASE_SPEED * TIME_STEP;
        if transform.translation.x < MIN_X {
            transform.translation.x = MIN_X
        }
        if transform.translation.x > MAX_X {
            transform.translation.x = MAX_X
        }
        if transform.translation.y < MIN_Y {
            transform.translation.y = MIN_Y
        }
        if transform.translation.y > MAX_Y {
            transform.translation.y = MAX_Y
        }
    }
}