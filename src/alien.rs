
#[derive(Component, Clone, Copy, Debug)]
pub struct Alien {
    kind: AlienKind,
    row: usize,
    col: usize,
    animation_frame: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlienKind {
    One,
    Two,
    Three
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