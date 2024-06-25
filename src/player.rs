use crate::{
    components::{Movable, Player, Velocity},
    GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE,
};
use bevy::prelude::*;

const LASER_VELOCITY_X: f32 = 0.;
const LASER_VELOCITY_Y: f32 = 0.2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system)
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
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2. + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Name::new("player"))
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. });
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            commands.spawn((
                SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y + PLAYER_SIZE.1 / 2., 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Movable { auto_despawn: true },
                Velocity {
                    x: LASER_VELOCITY_X,
                    y: LASER_VELOCITY_Y,
                },
                Name::new("laser"),
            ));
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
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
