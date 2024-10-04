use noise::{NoiseFn, Perlin};
use bevy::prelude::*;
use crate::tera_grid::Grid;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
           .add_systems(Startup, generate_terrain.after(setup));
    }
}

#[derive(Resource)]
pub struct PerlinGenerator(Perlin);
fn setup(mut commands: Commands) {
    let perlin: Perlin = Perlin::new(1_u32);
    commands.insert_resource(PerlinGenerator(perlin));
}

const EPSILON: f64 = 0.016;
fn generate_terrain(
    mut commands: Commands,
    grid: Res<Grid>,
    perlin: Res<PerlinGenerator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_size: u32 = grid.tile_size;
    for x in 0..100+grid.grid_width {
        for y in 0..100+grid.grid_height {
            let noise_value = perlin.0.get([(x as f64 + EPSILON) / 10.0, (y as f64 + EPSILON) / 10.0, 0.0]);
            let color = map_noise_to_color(noise_value as f32);

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(tile_size as f32, tile_size as f32))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz((x as f32 + 0.5) * tile_size as f32, (y as f32 + 0.5) * tile_size as f32, -5.0),
            ));

        }
    }
}

// Convert Perlin noise output to a color value
fn map_noise_to_color(value: f32) -> Color {
    let normalized = (value + 1.0) / 2.0; // Normalize to [0, 1]
    Color::hsla(normalized * 360.0, 1.0, 0.5, 1.0)
}

