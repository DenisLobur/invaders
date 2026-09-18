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
    input::{ButtonInput, keyboard::KeyCode},
    sprite::Sprite,
    state::{
        condition::in_state,
        state::{OnEnter, OnExit},
    },
    time::Time,
    transform::components::Transform,
};

use crate::{
    config::{
        PLAYER_SIZE, PLAYER_SPEED, PLAYER_X_POSITION, PLAYER_Y_POSITION, WINDOW_LEFT_BOUND,
        WINDOW_RIGHT_BOUND,
    },
    state::GameState,
};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 0.0, 0.5), PLAYER_SIZE),
        Transform::from_xyz(PLAYER_X_POSITION, PLAYER_Y_POSITION, 0.0),
        Player,
    ));
}

pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player_entity in &player_query {
        commands.entity(player_entity).despawn();
    }
}

pub fn move_palyer(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let left = input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA);
        let right = input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD);

        let direction: f32 = if left && !right {
            -1.0
        } else if right && !left {
            1.0
        } else {
            0.0
        };

        let movement = direction * PLAYER_SPEED * time.delta_secs();
        transform.translation.x += movement;

        let min_x = WINDOW_LEFT_BOUND + PLAYER_SIZE.x / 2.0;
        let max_x = WINDOW_RIGHT_BOUND - PLAYER_SIZE.x / 2.0;

        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(OnExit(GameState::Playing), cleanup_player);
        app.add_systems(Update, move_palyer.run_if(in_state(GameState::Playing)));
    }
}
