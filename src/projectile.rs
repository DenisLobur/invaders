use bevy::{
    app::{Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, common_conditions::input_just_pressed, keyboard::KeyCode},
    sprite::Sprite,
    state::condition::in_state,
    time::Time,
    transform::components::Transform,
};

use crate::{
    config::{BULLET_HEIGHT, BULLET_SIZE, BULLET_SPEED, PLAYER_HEIGHT},
    player::Player,
    state::GameState,
};

#[derive(Component)]
pub struct PlayerBullet;

pub fn spawn_bullet(mut commands: Commands, player_query: Query<&Transform, With<Player>>) {
    if let Ok(player_transform) = player_query.single() {
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;

        commands.spawn((
            Sprite::from_color(Color::srgb(0.0, 0.0, 0.5), BULLET_SIZE),
            Transform::from_xyz(
                player_x,
                player_y + PLAYER_HEIGHT / 2.0 + BULLET_HEIGHT / 2.0,
                0.0,
            ),
            PlayerBullet,
        ));
    }
}

// testing purposes
pub fn remove_bullet(mut commands: Commands, query: Query<Entity, With<PlayerBullet>>) {
    for bullet in query {
        commands.entity(bullet).despawn();
    }
}

pub fn move_bullet(mut query: Query<&mut Transform, With<PlayerBullet>>, time: Res<Time>) {
    for mut bullet in &mut query {
        bullet.translation.y += BULLET_SPEED * time.delta_secs();
    }
}

pub fn check_if_bullet_exists(
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    query: Query<Entity, With<PlayerBullet>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let bullet_exists = query.iter().next().is_some();

        if !bullet_exists {
            spawn_bullet(commands, player_query);
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            check_if_bullet_exists.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
            Update,
            remove_bullet.run_if(input_just_pressed(KeyCode::KeyS)),
        );
        app.add_systems(Update, move_bullet.run_if(in_state(GameState::Playing)));
    }
}
