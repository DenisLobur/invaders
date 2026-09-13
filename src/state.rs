// By attaching .run_if(in_state(GameState::Playing)) to gameplay systems
// (player movement, alien march, projectile physics), pausing requires zero conditional
// checks inside individual systems.

// OnEnter(GameState::Playing): Spawns the player, alien grid, and bunkers.

// OnExit(GameState::Playing) or
// OnEnter(GameState::GameOver): Cleans up bullets, enemies, and arena entities.

use bevy::{ecs::resource::Resource, state::state::States};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Paused,
    LevelClear,
    GameOver,
}

// Player's current score (defaults to 0).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Score(pub u32);

// Remaining player lives (defaults to 3).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lives(pub u32);

impl Default for Lives {
    fn default() -> Self {
        Self(3)
    }
}

// Current wave counter (defaults to 1).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level(pub u32);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}
