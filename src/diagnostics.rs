use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::{CameraPlugin, DragState, screen_to_world, MainCamera};
use crate::basics::ToWorldGrid;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PositionText;

pub struct DiagnosticsPlugin;

const FONT_PATH: &str = "fonts/NotoSansMono-Regular.ttf";

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CameraPlugin)
            .add_systems(Update, (update_position_text,))
            .add_systems(Startup, (setup_text,));
    }
}
fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 100.0,
                ..default()
            },
        )
    );

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 60.0,
                    ..default()
                },
            ),
        ]),
        PositionText,
    ));
}


fn update_position_text(
    mut camera_query: Query<(&Camera, &mut Transform, &OrthographicProjection), With<MainCamera>>, 
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Text, With<PositionText>>,
    state: ResMut<DragState>,
){
    let window = windows.single();
    let (_, camera_transform, orthographic_projection) = camera_query.single_mut();

    if let Some(position) = window.cursor_position() {
        if let Some(world_position) = screen_to_world(
            window,
            &camera_transform,
            position,
            orthographic_projection, // Pass this to the function
        ) {
            for mut text in &mut query {
                text.sections[0].value = format!("World : {} {}\n\
                                                  Window: {} {}\n\
                                                  Saved : {} {}",
                    world_position.to_world_grid(), world_position.to_int_string(),
                    position.to_world_grid(), &position.to_int_string(),
                    &state.initial_camera_pos.to_world_grid(), &state.initial_camera_pos.to_int_string());
            }

        }
    }
}

