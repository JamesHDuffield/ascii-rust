use bevy::prelude::*;

mod physics;
mod engine;
mod health;
mod turret;
mod bullet;
mod loot;
mod worth_points;
mod upgrades;

// Complex components
pub use physics::*;
pub use engine::*;
pub use health::*;
pub use turret::*;
pub use bullet::*;
pub use loot::*;
pub use worth_points::*;
pub use upgrades::*;

use crate::colour;

// Simple components
#[derive(Component)]
pub struct DisplayName(pub String);

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
pub enum UINode {
    Status,
    Equipment,
}

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

#[derive(Component)]
pub struct DespawnWithScene;

#[derive(Component)]
pub struct ExplodesOnDespawn {
    pub amount_min: u32,
    pub amount_max: u32,
    pub spread: f32,
    pub colour: Color,
    pub duration_min: f32,
    pub duration_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

impl ExplodesOnDespawn {
    pub fn default() -> ExplodesOnDespawn {
        ExplodesOnDespawn {
            amount_min: 1,
            amount_max: 1,
            colour: colour::RED,
            duration_min: 0.3,
            duration_max: 0.4,
            size_min: 40.0,
            size_max: 40.0,
            spread: 10.0,
        }
    }
}