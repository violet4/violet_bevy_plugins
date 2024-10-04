
use bevy::prelude::*;

pub struct TeraGridPlugin;


impl Plugin for TeraGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid)
            .add_systems(Update, create_grid);
    }
}

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new(10, 5, 64));
}

fn create_grid(
    grid: Res<Grid>,
    gizmos: Gizmos,
) {
    grid.draw(gizmos);
}
#[derive(Component, Resource)]
pub struct Grid {
    pub grid_width: u32,
    pub grid_height: u32,
    pub tile_size: u32,
}

impl Grid {
    pub fn new(grid_width: u32, grid_height: u32, tile_size: u32) -> Self {
        Grid {
            grid_width,
            grid_height,
            tile_size,
        }
    }
// vertical bar y increase by half tile_size
    // horizontal bar x increase by half tile_size
    pub fn draw(
        &self,
        mut gizmos: Gizmos,
    ) {

        let h_segment = Segment2d::new(Dir2::X, ((self.grid_width - 1) * self.tile_size) as f32);
        let v_segment = Segment2d::new(Dir2::Y, ((self.grid_height - 1) * self.tile_size) as f32);

        let red = Color::srgb_u8(50, 0, 0);

        for x in 0..self.grid_width {
            for y in 0..self.grid_height  {
                gizmos.primitive_2d(
                    &v_segment,
                    Isometry2d::new(
                        Vec2::new(
                            (x * self.tile_size) as f32,
                            v_segment.half_length,
                        ),
                        (0.).into(),
                    ),
                    red,
                );

                gizmos.primitive_2d(
                    &h_segment,
                    Isometry2d::new(
                        Vec2::new(
                            h_segment.half_length,
                            (y * self.tile_size) as f32,
                        ),
                        (0.).into(),
                    ),
                    red,
                );
            }
        }
    }
    pub fn get_grid_coord(&self, world_pos: Vec2) -> [i32; 2] {
        [
            (world_pos.x / self.tile_size as f32).floor() as i32,
            (world_pos.y / self.tile_size as f32).floor() as i32,
        ]
    }

}
