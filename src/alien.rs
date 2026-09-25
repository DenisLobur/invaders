use bevy::app::{Plugin, Startup};
use bevy::color::Color;
use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::Resource;
use bevy::sprite::Sprite;
use bevy::state::state::OnEnter;
use bevy::transform::components::Transform;

use crate::config::{
    ALIEN_COLUMN, ALIEN_INITIAL_FORMATION_X, ALIEN_INITIAL_FORMATION_Y, ALIEN_ROW, ALIEN_SIZE,
    ALIEN_SPACING_HORIZONTAL, ALIEN_SPACING_VERTICAL,
};
use crate::state::GameState;

#[derive(Component, Clone, Copy, Debug)]
pub struct Alien {
    kind: AlienKind,
    row: usize,
    col: usize,
    animation_frame: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlienKind {
    Squid,
    Crab,
    Octopus,
}

#[derive(Resource)]
struct Formation {
    direction: f32,
    march_timer: f32,
    step_interval: f32,
    descent_pending: bool,
    animation_frame: u8,
    remaining: usize,
}

fn spawn_alien(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), ALIEN_SIZE),
        Transform::from_xyz(0.0, 240.0, 0.0),
        Alien {
            kind: AlienKind::Crab,
            row: 0,
            col: 0,
            animation_frame: 0,
        },
    ));
}

fn spawn_formation(mut commands: Commands) {
    for row in 0..ALIEN_ROW as usize {
        for col in 0..ALIEN_COLUMN as usize {
            let x = ALIEN_INITIAL_FORMATION_X + col as f32 * ALIEN_SPACING_HORIZONTAL;
            let y = ALIEN_INITIAL_FORMATION_Y + row as f32 * ALIEN_SPACING_VERTICAL;

            let kind = match row {
                0 => AlienKind::Squid,
                1 | 2 => AlienKind::Crab,
                _ => AlienKind::Octopus,
            };

            commands.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), ALIEN_SIZE),
                Transform::from_xyz(x, y, 0.0),
                Alien {
                    kind,
                    row,
                    col,
                    animation_frame: 0,
                },
            ));
        }
    }
}

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        //app.add_systems(OnEnter(GameState::Playing), spawn_alien);
        app.add_systems(Startup, spawn_formation);
    }
}