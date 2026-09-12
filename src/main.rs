use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Space Invaders".into(),
            resolution: WindowResolution::new(800, 600),
            ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .run();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver
}
