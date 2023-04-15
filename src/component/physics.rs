use bevy::prelude::*;

#[derive(Component)]
pub struct Physics {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub drag: f32
}

impl Physics {
    pub fn add_force(&mut self, force: Vec2) -> () {
        self.acceleration += force;
    }
}