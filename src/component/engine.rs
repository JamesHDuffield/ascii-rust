use bevy::prelude::*;

#[derive(Component)]
pub struct Engine {
    pub target: Option<Vec2>,
    pub power: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub depower_factor: f32
}

impl Engine {

    pub fn new(power: f32, max_speed: f32) -> Engine {
        Engine { target: None, power, speed: 0.0, max_speed, depower_factor: 5.0 }
    }

}