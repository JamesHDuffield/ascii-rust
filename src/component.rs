mod physics;
mod engine;
mod health;
mod turret;
mod bullet;
mod spawner;

use bevy::prelude::*;

// Complex components

pub use physics::*;
pub use engine::*;
pub use health::*;
pub use turret::*;
pub use bullet::*;
pub use spawner::*;

// Simple components
#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct AI;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BaseGlyphRotation {
    pub rotation: Quat,
}

#[derive(Component)]
pub struct UINode;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Reflect, Component)]
pub struct Owner(pub Entity);

#[derive(Component)]
pub struct Targettable;

// Turret components

// Bullet Components
#[derive(Component)]
pub struct LaserRender;
