mod physics;
mod engine;

use bevy::prelude::*;

// Complex components

pub use physics::*;
pub use engine::*;

// Simple components

#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BaseGlyphRotation {
    pub rotation: Quat,
}