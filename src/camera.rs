use bevy::prelude::*;
use bevy::render::camera::CameraProjection;
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
    mut camera_query: Query<(&Camera, &mut Transform, &OrthographicProjection), With<MainCamera>>, 
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let (_, mut camera_transform, orthographic_projection) = camera_query.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            if let Some(world_position) = screen_to_world(
                window,
                &camera_transform,
                position,
                orthographic_projection,
            ) {
                state.initial_camera_pos = world_position;
            }
        }
        state.is_dragging = true;
    } else if mouse_button_input.just_released(MouseButton::Left) {
        state.is_dragging = false;
    }

    if state.is_dragging {
        if let Some(position) = window.cursor_position() {
            if let Some(world_position) = screen_to_world(
                window,
                &camera_transform,
                position,
                orthographic_projection,
            ) {
                let delta = world_position - state.initial_camera_pos;
                camera_transform.translation.x -= delta.x;
                camera_transform.translation.y -= delta.y;
            }
        }
    }
}

pub fn screen_to_world(
    window: &Window,
    camera_transform: &Transform,
    mouse_position: Vec2,
    projection: &OrthographicProjection,
) -> Option<Vec2> {
    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position to Normalized Device Coordinates (NDC).
    // i think the primary issue begins and propagates here because we make a faulty assumption
    // about the stability and usability of mouse_position.
    let ndc = (mouse_position / window_size) * 2.0 - Vec2::ONE;

    // Flip the Y-axis for Bevy's coordinate system
    let ndc_flipped = Vec4::new(ndc.x, -ndc.y, 0.0, 1.0);

    let view_to_world = camera_transform.compute_matrix() * projection.get_clip_from_view().inverse();

    let world_position = view_to_world * ndc_flipped;

    Some(world_position.truncate().xy())
}
