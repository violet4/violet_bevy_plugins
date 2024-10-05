use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

use crate::tera_grid::Grid;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub zoom: f32,
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_scroll_events, handle_key_events))
            .init_resource::<PlayerInput>();
    }
}

fn handle_key_events(
    key_events: Res<ButtonInput<KeyCode>>,
    mut grid: ResMut<Grid>,
) {
    if key_events.just_pressed(KeyCode::KeyG) {
        grid.toggle_grid();
    }
}
fn fetch_scroll_events(
    mut scroll_events: EventReader<MouseWheel>,
    mut player_input: ResMut<PlayerInput>,
) {
    for event in scroll_events.read() {
        let scroll = match event.unit {
            MouseScrollUnit::Line => {
                if event.y > 0.0 {
                    -1.0
                } else {
                    1.0
                }
            }
            MouseScrollUnit::Pixel => {
                if event.y > 0.0 {
                    -1.0
                } else {
                    1.0
                }
            }
        };
        player_input.zoom -= scroll * player_input.zoom / 8.0;
    }

}
