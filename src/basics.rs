use bevy::prelude::*;



pub trait ToWorldGrid {
    fn to_world_grid(&self) -> String;
    fn to_int_string(&self) -> String;
}
impl ToWorldGrid for Vec2 {
    fn to_world_grid(&self) -> String {
        let xy = self.xy();
        format!("[{:.0}, {:.0}]", xy.x / 32.0, xy.y / 32.0)
    }
    fn to_int_string(&self) -> String {
        let xy = self.xy();
        format!("[{:.0}, {:.0}]", xy.x, xy.y)
    }
}
