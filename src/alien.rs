use bevy::app::{Plugin, Update};
use bevy::color::Color;
use bevy::ecs::query::With;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::Resource;
use bevy::sprite::Sprite;
use bevy::state::condition::in_state;
use bevy::state::state::OnEnter;
use bevy::time::{Time, Timer, TimerMode};
use bevy::transform::components::Transform;

use crate::config::{
    ALIEN_COLUMN, ALIEN_DESCENT_STEP_DISTANCE, ALIEN_INITIAL_FORMATION_X, ALIEN_INITIAL_FORMATION_Y, ALIEN_ROW, ALIEN_SIZE, ALIEN_SPACING_HORIZONTAL, ALIEN_SPACING_VERTICAL, WINDOW_LEFT_BOUND, WINDOW_RIGHT_BOUND,
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
    march_timer: Timer,
    step_interval: f32,
    descent_pending: bool,
    animation_frame: u8,
    remaining: usize,
}

impl Default for Formation {
    fn default() -> Self {
        Self {
            direction: 1.0,
            march_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            step_interval: ALIEN_SIZE.x,
            descent_pending: false,
            animation_frame: 0,
            remaining: 55,
        }
    }
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

fn move_formation(
    time: Res<Time>,
    mut formation: ResMut<Formation>,
    mut query: Query<&mut Transform, With<Alien>>,
) {
    formation.march_timer.tick(time.delta());
    let mut leftmost = f32::INFINITY;
    let mut rightmost = f32::NEG_INFINITY;

    for transform in query.iter() {
        leftmost = leftmost.min(transform.translation.x);
        rightmost = rightmost.max(transform.translation.x);
    }

    if !leftmost.is_finite() {
        return;
    }

    let half_width = ALIEN_SIZE.x / 2.0;
    
    let dx = formation.direction * formation.step_interval;
    
    let next_left = leftmost - half_width + dx;
    let next_right = rightmost + half_width + dx;
    let cross_edge = next_left < WINDOW_LEFT_BOUND || next_right > WINDOW_RIGHT_BOUND;
    
    if formation.march_timer.just_finished() {
        if cross_edge {
                formation.direction *= -1.0;
                formation.descent_pending = true;                
            } else {     
                formation.descent_pending = false;           
            }

        for mut transform in query.iter_mut() {
            if formation.descent_pending {
                transform.translation.y += ALIEN_DESCENT_STEP_DISTANCE;
            } else {
                transform.translation.x += formation.direction * formation.step_interval;
            }
        }
    }
}

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        //app.add_systems(OnEnter(GameState::Playing), spawn_alien);
        app.insert_resource(Formation::default());
        app.add_systems(OnEnter(GameState::Playing), spawn_formation);
        app.add_systems(Update, move_formation.run_if(in_state(GameState::Playing)));
    }
}