use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn distance(&self, origin: Vec2) -> f32 {
        let x1 = self.0 as f32;
        let y1 = self.1 as f32;
        let xy = origin.xy();
        let x2 = xy.x;
        let y2 = xy.y;
        ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
    }
}
