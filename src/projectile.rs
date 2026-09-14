use bevy::{
    ecs::{
        system::{Commands},
    },
};

use crate::{
    config::{
        BULLET_SIZE
    },
    state::GameState,
};

#[derive(Component)]
pub struct Bullet;

pub fn spawn_bullet(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 0.0, 0.5), BULLET_SIZE),
        Transform::from_xyz(PLAYER_X_POSITION / 2.0, PLAYER_Y_POSITION, 0.0),
        Bullet,
    ))
}

