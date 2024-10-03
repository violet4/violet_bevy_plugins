use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::{CameraPlugin, DragState, screen_to_world, MainCamera};
use crate::basics::ToWorldGrid;
use crate::tera_grid::Grid;
// #[derive(Component)]
// struct FpsText;

#[derive(Component)]
struct PositionText;


const FONT_PATH: &str = "fonts/NotoSansMono-Regular.ttf";
fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 16.0,
        ..default()
    }
}

pub struct DiagnosticsPlugin;

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
            "", get_text_style(&asset_server)
        )
    );

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                get_text_style(&asset_server),
            ),
        ]),
        PositionText,
    ));
}


fn update_position_text(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), With<MainCamera>>, 
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Text, With<PositionText>>,
    state: ResMut<DragState>,
    grid: Res<Grid>,
){
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single_mut();

    if let Some(position) = window.cursor_position() {
        if let Some(world_pos) = screen_to_world(camera, &camera_transform, window) {
            for mut text in &mut query {
                text.sections[0].value = format!("World : {:?} {}\n\
                                                  Window: {} {}\n\
                                                  Saved : {} {}",
                    grid.get_grid_coord(world_pos), world_pos.to_int_string(),
                    position.to_world_grid(), &position.to_int_string(),
                    &state.initial_camera_pos.to_world_grid(), &state.initial_camera_pos.to_int_string());
            }

        }
    }
}

