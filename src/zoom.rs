use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::camera::MainCamera;
use crate::player_input::PlayerInput;


pub struct ZoomPlugin;

impl Plugin for ZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, zoom_camera);
    }
}


fn zoom_camera(
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut projection = match q_projection.get_single_mut() {
        Ok(p) => dbg!(p),
        Err(_) => return,
    };
    player_input.zoom = player_input.zoom.clamp(0.1, 20.0);
    projection.scaling_mode = ScalingMode::WindowSize(player_input.zoom);
}

