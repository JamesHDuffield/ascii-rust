use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Physics {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub drag: f32
}

impl Physics {

    pub fn new(drag: f32) -> Physics {
        Physics { acceleration: Vec2 { x: 0.0, y: 0.0 }, velocity: Vec2 { x: 0.0, y: 0.0 }, drag }
    }

    pub fn add_force(&mut self, force: Vec2) -> () {
        self.acceleration += force;
    }
}