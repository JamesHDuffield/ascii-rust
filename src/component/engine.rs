use bevy::prelude::*;

#[derive(Component)]
pub struct Engine {
    pub target: Option<Vec2>,
    pub method: EngineMethod,
    pub power: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub depower_factor: f32,
    pub steering_factor: f32,
}

impl Default for Engine {
    fn default() -> Self { Engine { target: None, method: EngineMethod::Approach, power: 10.0, speed: 0.0, max_speed: 10.0, depower_factor: 5.0, steering_factor: 20.0 } }
}

impl Engine {

    pub fn new(power: f32, max_speed: f32) -> Engine {
        Engine { target: None, method: EngineMethod::Approach, power, speed: 0.0, max_speed, depower_factor: 5.0, steering_factor: 20.0 }
    }

    pub fn new_with_steering(power: f32, max_speed: f32, steering_factor: f32) -> Engine {
        Engine { target: None, method: EngineMethod::Approach, power, speed: 0.0, max_speed, depower_factor: 5.0, steering_factor }
    }

}

pub enum EngineMethod {
    Approach,
    Keep(f32),
    Orbit(f32),
}