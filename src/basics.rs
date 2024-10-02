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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let v = Vec2 {x: 0., y: 0.};
        assert_eq!(v.to_world_grid(), "[0, 0]");
    }
    #[test]
    fn another_test() {
        assert_eq!(1 + 1, 2);
    }
}


