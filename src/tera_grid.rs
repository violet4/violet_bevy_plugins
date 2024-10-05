#![allow(unused_mut, unused)]

use std::ops::Div;

use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::*};

pub struct TeraGridPlugin;

impl Plugin for TeraGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, |grid: Res<Grid>, mut gizmos: Gizmos| {
                grid.draw(gizmos)
            });
    }
}
fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.insert_resource(Grid::new(100, 100, 64, false));
}

// gcfn draw(grid: Res<Grid>, mut gizmos: Gizmos) {
//     grid.draw(gizmos);
// }

#[derive(Component, Resource)]
pub struct Grid {
    width: u32,
    height: u32,
    tile_size: u32,
    show_border: bool,
    grid_segments: GridSegment,
    grid_color: Color,
}

struct GridSegment {
    h_segment: Segment2d,
    v_segment: Segment2d,
}

impl Grid {
    pub fn new(width: u32, height: u32, tile_size: u32, show_border: bool) -> Self {
        Grid {
            width,
            height,
            tile_size,
            show_border,
            grid_segments: GridSegment {
                h_segment: Segment2d::new(Dir2::X, (width * tile_size) as f32),
                v_segment: Segment2d::new(Dir2::Y, (height * tile_size) as f32),
            },
            grid_color: Color::srgb_u8(0, 0, 255),
        }
    }

    pub fn draw(&self, mut gizmos: Gizmos) {
        if !self.show_border {
            return;
        }

        for x in 0..=self.width {
            let x_iso_vec = Vec2::new(
                (x * self.tile_size) as f32,
                self.grid_segments.v_segment.half_length,
            );
            gizmos.primitive_2d(
                &self.grid_segments.v_segment,
                Isometry2d::new(x_iso_vec, Rot2::default()),
                self.grid_color,
            );

        }
        for y in 0..=self.height {
            let y_iso_vec = Vec2::new(
                self.grid_segments.h_segment.half_length,
                (y * self.tile_size) as f32,
            );

            gizmos.primitive_2d(
                &self.grid_segments.h_segment,
                Isometry2d::new(y_iso_vec, Rot2::default()),
                self.grid_color,
            );
        }
    }
    pub fn toggle_grid(&mut self) {
        self.show_border = !self.show_border;
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_grid_coord_from_global(&self, world_pos: Vec2) -> [f32; 2] {
        [
            (world_pos.x / self.tile_size as f32).floor(),
            (world_pos.y / self.tile_size as f32).floor(),
        ]
    }

    pub fn get_tile_center_from_global(&self, world_pos: Vec2) -> [f32; 2] {
        self.get_grid_coord_from_global(world_pos)
            .map(|c| c + self.tile_size as f32 / 2.)
    }

    pub fn get_global_from_local(&self, x: f32, y: f32) -> Vec3 {
        Vec3::new(
            x * (self.tile_size) as f32 + (self.tile_size / 2) as f32,
            y * (self.tile_size) as f32 + (self.tile_size / 2) as f32,
            0.,
        )
    }

    pub fn get_new_pos(&self, current_loc: Vec3, x_amount: f32, y_amount: f32) -> Vec3 {
        current_loc
            + Vec3::new(
                x_amount * self.tile_size as f32,
                y_amount * self.tile_size as f32,
                0.,
            )
    }
}

