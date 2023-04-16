mod physics;
mod engine;
mod health;

use bevy::prelude::*;

// Complex components

pub use physics::*;
pub use engine::*;
pub use health::*;

// Simple components
#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BaseGlyphRotation {
    pub rotation: Quat,
}

#[derive(Component)]
pub struct UINode;