use bevy::prelude::*;

use crate::tera_grid::Grid;


pub trait ToWorldGrid {
    fn to_world_grid(&self, grid: &Grid) -> String;
    fn to_int_string(&self) -> String;
}
impl ToWorldGrid for Vec2 {
    fn to_world_grid(&self, grid: &Grid) -> String {
        let xy = self.xy();
        format!("[{:.0}, {:.0}]", xy.x / grid.width_f32(), xy.y / grid.height_f32())
    }
    fn to_int_string(&self) -> String {
        let xy = self.xy();
        format!("[{:.0}, {:.0}]", xy.x, xy.y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn another_test() {
        assert_eq!(1 + 1, 2);
    }
}


