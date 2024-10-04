use bevy::prelude::*;
use bevy::render::camera::ViewportConversionError;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (camera_drag_system,))
        .add_systems(Startup, setup_camera)
        .init_resource::<DragState>();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct DragState {
    is_dragging: bool,
    pub initial_camera_pos: Vec2,
}

impl Default for DragState {
    fn default() -> DragState {
        DragState {
            is_dragging: false,
            initial_camera_pos: Vec2::new(0., 0.)
        }
    }
}

fn camera_drag_system(
    mut state: ResMut<DragState>,
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut Transform), With<MainCamera>>, 
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let (camera, transform, mut camera_transform) = camera_query.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = screen_to_world(camera, transform, window) {
            state.initial_camera_pos = world_pos;
        }
        state.is_dragging = true;
    } else if mouse_button_input.just_released(MouseButton::Left) {
        state.is_dragging = false;
    }

    if state.is_dragging {
        if let Some(world_pos) = screen_to_world(camera, transform, window) {
                let delta = world_pos - state.initial_camera_pos;
                camera_transform.translation.x -= delta.x;
                camera_transform.translation.y -= delta.y;
        }
    }
}

pub fn screen_to_world(
    camera: &Camera,
    transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    let cursor = window.cursor_position()?;

    match camera.viewport_to_world_2d(transform, cursor) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
