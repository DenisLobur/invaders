use crate::player::PlayerPlugin;
use crate::projectile::{
    ProjectilePlugin, check_if_bullet_exists, check_if_bullet_outside_arena, move_bullet,
};
use crate::state::*;
use crate::{config::*, player::move_player};
use bevy::{prelude::*, window::WindowResolution};

pub mod config;
pub mod player;
pub mod projectile;
pub mod state;
pub mod ui;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameFlowSet {
    PlayerMove,
    BulletSpawn,
    BulletMove,
    BulletCleanup,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invaders".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(ProjectilePlugin)
        .init_resource::<Score>()
        .init_resource::<Lives>()
        .init_resource::<Level>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, ui::setup_hud)
        .add_systems(Update, ui::update_score_hud)
        .configure_sets(
            Update,
            (
                GameFlowSet::PlayerMove,
                GameFlowSet::BulletSpawn,
                GameFlowSet::BulletMove,
                GameFlowSet::BulletCleanup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            move_player
                .run_if(in_state(GameState::Playing))
                .in_set(GameFlowSet::PlayerMove),
        )
        .add_systems(
            Update,
            check_if_bullet_exists
                .run_if(in_state(GameState::Playing))
                .in_set(GameFlowSet::BulletSpawn),
        )
        .add_systems(
            Update,
            move_bullet
                .run_if(in_state(GameState::Playing))
                .in_set(GameFlowSet::BulletMove),
        )
        .add_systems(
            Update,
            check_if_bullet_outside_arena
                .run_if(in_state(GameState::Playing))
                .in_set(GameFlowSet::BulletCleanup),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
