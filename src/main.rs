use crate::config::*;
use crate::player::PlayerPlugin;
use crate::projectile::ProjectilePlugin;
use crate::state::*;
use bevy::{prelude::*, window::WindowResolution};

pub mod config;
pub mod player;
pub mod projectile;
pub mod state;
pub mod ui;

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
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
