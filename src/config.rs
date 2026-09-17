use bevy::math::Vec2;

// Window & Arena
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_LEFT_BOUND: f32 = -360.0;
pub const WINDOW_RIGHT_BOUND: f32 = 360.0;
pub const WINDOW_TOP_BOUND: f32 = -280.0;
pub const WINDOW_BOTTOM_BOUND: f32 = 280.0;
pub const INVASION_FLOOR_LINE: f32 = 248.0;

// Player
pub const PLAYER_SIZE: Vec2 = Vec2::new(48.0, 32.0);
pub const PLAYER_HEIGHT: f32 = PLAYER_SIZE.y;
pub const PLAYER_Y_POSITION: f32 = -240.0;
pub const PLAYER_X_POSITION: f32 = 0.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_LEFT_MOVEMENT_BOUND: f32 = WINDOW_LEFT_BOUND;
pub const PLAYER_RIGHT_MOVEMENT_BOUND: f32 = WINDOW_RIGHT_BOUND;

// Aliens & Formation
pub const ALIEN_SIZE: Vec2 = Vec2::new(32.0, 24.0);
pub const ALIEN_ROW: u32 = 5;
pub const ALIEN_COLUMN: u32 = 11; // grid structure 5 rows x 11 columns
pub const ALIEN_SPACING_HORIZONTAL: f32 = 48.0;
pub const ALIEN_SPACING_VERTICAL: f32 = 36.0;
pub const ALIEN_INITIAL_FORMATION_X: f32 = -240.0;
pub const ALIEN_INITIAL_FORMATION_Y: f32 = 200.0;
pub const ALIEN_DESCENT_STEP_DISTANCE: f32 = -16.0;

// Projectiles (Player bullets & Aliens bombs)
pub const BULLET_SIZE: Vec2 = Vec2::new(4.0, 12.0);
pub const BULLET_HEIGHT: f32 = BULLET_SIZE.y;
pub const BULLET_SPEED: f32 = 300.0;
pub const BOMB_SIZE: f32 = 250.0;
pub const BOMB_SPEED: f32 = 250.0;
pub const PROJECTILE_DESPAWN_THRESHOLD_TOP: f32 = WINDOW_TOP_BOUND;
pub const PROJECTILE_DESPAWN_THRESHOLD_BOTTOM: f32 = WINDOW_BOTTOM_BOUND;

// Bunkers & UFO
pub const BUNKER_COUNT: u32 = 4;
pub const BUNKER_SIZE: f32 = 48.0;
pub const BUNKER_SPACING: f32 = 40.0;
pub const BUNKER_POSITION_TOP: f32 = -200.0;
pub const UFO_SIZE: Vec2 = Vec2::new(32.0, 24.0);
pub const UFO_SPEED: f32 = 300.0;
pub const UFO_SPAWN_INTERVAL: f32 = 120.0;
pub const UFO_POSITION_TOP: f32 = -275.0;
