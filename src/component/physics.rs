use bevy::prelude::*;

#[derive(Component)]
pub struct Physics {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub drag: f32,
    pub face_velocity: bool
}

impl Physics {

    pub fn new(drag: f32) -> Physics {
        Physics { drag, ..Default::default() }
    }

    pub fn add_force(&mut self, force: Vec2) -> () {
        self.acceleration += force;
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self { acceleration: Vec2::ZERO, velocity: Vec2::ZERO, drag: 0.0, face_velocity: true }
    }
}