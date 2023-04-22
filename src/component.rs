use bevy::prelude::*;

mod physics;
mod engine;
mod health;
mod turret;
mod bullet;
mod spawner;
mod loot;

// Complex components
pub use physics::*;
pub use engine::*;
pub use health::*;
pub use turret::*;
pub use bullet::*;
pub use spawner::*;
pub use loot::*;

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

#[derive(PartialEq)]
pub enum Allegiance {
    PLAYER,
    ENEMY,
}

#[derive(Component)]
pub struct Targettable(pub Allegiance);

#[derive(Component)]
pub struct WillTarget(pub Vec<Allegiance>);

#[derive(Component)]
pub struct ExplosionRender {
    pub origin: Vec2,
    pub radius: f32,
    pub ttl: Timer,
}

#[derive(Component)]
pub struct ShouldDespawn;

// Turret components

// Bullet Components
#[derive(Component)]
pub struct LaserRender;
