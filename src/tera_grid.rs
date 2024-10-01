#![allow(unused_mut, unused)]

use std::ops::Div;

use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::*};

pub struct TeraGridPlugin;

// cannot find derive macro `SystemLabel` in this scope

impl Plugin for TeraGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, create_grid);
    }
}

fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.insert_resource(Grid::new(5, 5, 32, true));
}

fn create_grid(
    grid: Res<Grid>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    grid.draw(gizmos, commands, meshes, materials);
}
#[derive(Component, Resource)]
pub struct Grid {
    width: u32,
    height: u32,
    tile_size: u32,
    show_border: bool,
}

impl Grid {
    pub fn new(width: u32, height: u32, tile_size: u32, show_border: bool) -> Self {
        Grid {
            width,
            height,
            tile_size,
            show_border,
        }
    }

    // pub fn translate_pos(&self, x: f32, y: f32, seg: &Segment2d) -> Vec2 {
    //     if seg.direction == Dir2::X {
    //         Vec2::new(x + seg.half_length, y)
    //     } else {
    //         Vec2::new(x, y + seg.half_length)
    //     }
    // }

    pub fn draw(
        &self,
        mut gizmos: Gizmos,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // let mesh = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::new(1., 1000.))));

        let h_segment = Segment2d::new(Dir2::X, ((self.width - 1) * self.tile_size) as f32);
        let v_segment = Segment2d::new(Dir2::Y, ((self.height - 1) * self.tile_size) as f32);

        let grey = Color::srgb_u8(100, 100, 100);
        let red = Color::srgb_u8(255, 0, 0);

        for x in 0..self.width {
            for y in 0..self.height {
                gizmos.primitive_2d(
                    &v_segment,
                    Vec2::new(
                        (x * self.tile_size) as f32,
                        v_segment.half_length,
                    ),
                    0.,
                    red,
                );

                gizmos.primitive_2d(
                    &h_segment,
                    Vec2::new(
                        h_segment.half_length,
                        (y * self.tile_size) as f32,
                    ),
                    0.,
                    red,
                );
            }
        }
        //
        // for i in 0..=self.width.div(self.tile_size) as i32 {
        //     gizmos.primitive_2d(
        //         &v_segment,
        //         // Vec2::new((i as f32 * self.tile_size), h_segment.half_length),
        //         // Vec2::new((i as f32 * self.tile_size), 0.),
        //         self.translate_pos((i as f32) * self.tile_size, 0., &v_segment),
        //         0.,
        //         grey,
        //     );
        // }
        //
        // for i in 0..=self.height.div(self.tile_size) as i32 {
        //     gizmos.primitive_2d(
        //         &h_segment,
        //         self.translate_pos(0., (i as f32) * self.tile_size, &h_segment),
        //         0.,
        //         grey,
        //     );
        // }
        //
        // for i in 0..self.height.div(self.tile_size) as i32 {
        //     gizmos.primitive_2d(
        //         &h_segment,
        //         // Vec2::new((i as f32 * self.tile_size), h_segment.half_length),
        //         Vec2::new((i as f32 * self.tile_size), 0.),
        //         0.,
        //         Color::srgb_u8(255, 0, 0),
        //     );
        // }

        // commands.spawn(MaterialMesh2dBundle {
        //     mesh,
        //     material: materials.add(Color::srgb_u8(255, 67, 176)),
        //     transform: Transform::from_xyz(0., 0., 0.),
        //     ..default()
        // });

        // for w in 0..self.width / self.tile_size {
        //     let mesh = Mesh2dHandle(
        //         meshes.add(Rectangle::from_size(Vec2::new(1., self.tile_size as f32))),
        //     );
        //     commands.spawn(MaterialMesh2dBundle {
        //         mesh,
        //         material: materials.add(Color::srgb_u8(100, 0, 0)),
        //         transform: Transform::from_xyz((w * self.tile_size) as f32, 0., 0.),
        //         ..default()
        //     });
        // }
        //
        // for h in 0..self.width / self.tile_size {
        //     let mesh = Mesh2dHandle(
        //         meshes.add(Rectangle::from_size(Vec2::new(self.tile_size as f32, 1.))),
        //     );
        //     commands.spawn(MaterialMesh2dBundle {
        //         mesh,
        //         material: materials.add(Color::srgb_u8(0, 100, 0)),
        //         transform: Transform::from_xyz(0., (h * self.tile_size) as f32, 0.),
        //         ..default()
        //     });
        // }
    }
}
