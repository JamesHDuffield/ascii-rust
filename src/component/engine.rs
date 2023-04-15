use bevy::prelude::*;

#[derive(Component)]
pub struct Engine {
    pub target: Option<Vec2>,
    pub power: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub depower_factor: f32
}