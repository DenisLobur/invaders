use bevy::{
    app::Plugin,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    sprite::Sprite,
    state::state::{OnEnter, OnExit},
    transform::components::Transform,
};

use crate::{
    config::{PLAYER_SIZE, PLAYER_X_POSITION, PLAYER_Y_POSITION},
    state::GameState,
};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 0.0, 1.0), PLAYER_SIZE),
        Transform::from_xyz(PLAYER_X_POSITION, PLAYER_Y_POSITION, 0.0),
        Player,
    ));
}

pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player_entity in &player_query {
        commands.entity(player_entity).despawn();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(OnExit(GameState::Playing), cleanup_player);
    }
}
