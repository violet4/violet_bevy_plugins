use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::diagnostic::*;

use crate::camera::{CameraPlugin, screen_to_world};
use crate::basics::ToWorldGrid;
use crate::tera_grid::Grid;

#[derive(Component)]
struct PositionText;


const FONT_PATH: &str = "fonts/NotoSansMono-Regular.ttf";
fn get_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load(FONT_PATH),
        font_size: 16.0,
        ..default()
    }
}

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((CameraPlugin, FrameTimeDiagnosticsPlugin))
            .add_systems(Update, (update_position_text,))
            .add_systems(Startup, (setup_text,));
    }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        Text::new("FPS: "),
        get_text_font(&asset_server),
        PositionText,
    ));
}


fn update_position_text(
    mut camera_query: Query<(&Camera, &mut GlobalTransform)>, 
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Text, With<PositionText>>,
    grid: Res<Grid>,
    diagnostic: Res<DiagnosticsStore>,
){
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single_mut();

    let mut fps_val = 0.0;
    if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|f| f.smoothed()) {
        fps_val = fps.trunc();
    }
    if let Some(window_position) = window.cursor_position() {
        if let Some(world_coordinates_f32) = screen_to_world(camera, &camera_transform, window) {
            for mut text in &mut query {
                text.0 = format!("World : {:?} {}\n\
                                    Window: {}\n\
                                    FPS: {}",
                    // world
                    grid.from_global(world_coordinates_f32), world_coordinates_f32.to_int_string(),
                    // window
                    &window_position.to_int_string(),
                    // fps
                    fps_val);
            }

        }
    }
}

