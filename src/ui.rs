use bevy::{
    color::Color,
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    text::{TextColor, TextFont},
    ui::{Node, Val, widget::Text},
    utils::default,
};

use crate::state::Score;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        Text::new("SCORE: 0"),
        TextFont {
            font_size: bevy::text::FontSize::Px(40.0),
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.8, 0.2)),
        ScoreText,
    ));
}

pub fn update_score_hud(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            **text = format!("SCORE: {:04}", score.0);
        }
    }
}
