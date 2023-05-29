use bevy::prelude::*;

mod bullet;
mod engine;
mod health;
mod loot;
mod physics;
mod turret;
mod worth_points;

// Complex components
pub use bullet::*;
pub use engine::*;
pub use health::*;
pub use loot::*;
pub use physics::*;
pub use turret::*;
pub use worth_points::*;

use crate::util::Colour;

// Bundles
#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub glyph: Text2dBundle,
    pub physics: Physics,
    pub engine: Engine,
    pub health: Health,
    pub collider: Collider,
    pub targettable: Targettable,
    pub will_target: WillTarget,
    pub despawn_with_scene: DespawnWithScene,
    pub explodes_on_despawn: ExplodesOnDespawn,
    pub hit_flash: HitFlash,
}

// Simple components
#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BaseGlyphRotation {
    pub rotation: Quat,
}

#[derive(Component, Default)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component)]
pub struct ExpandingCollider {
    pub final_radius: f32,
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

impl Default for Targettable {
    fn default() -> Self {
        Targettable(Allegiance::ENEMY)
    }
}

#[derive(Component)]
pub struct WillTarget(pub Vec<Allegiance>);

impl Default for WillTarget {
    fn default() -> Self {
        WillTarget(vec![Allegiance::PLAYER])
    }
}

#[derive(Component)]
pub struct ExplosionRender {
    pub origin: Vec2,
    pub radius: f32,
    pub ttl: Timer,
    pub fade_out: bool,
}

#[derive(Component)]
pub struct ShouldDespawn;

#[derive(Component, Default)]
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

impl Default for ExplodesOnDespawn {
    fn default() -> Self {
        ExplodesOnDespawn {
            amount_min: 1,
            amount_max: 1,
            colour: Colour::RED,
            duration_min: 0.3,
            duration_max: 0.4,
            size_min: 40.0,
            size_max: 40.0,
            spread: 10.0,
        }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub trauma: f32,
    pub decay: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay: 20.0,
        }
    }
}

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub flash_colour: Color,
    pub original_colour: Option<Color>,
}

impl HitFlash {
    pub fn hit(&mut self) {
        self.timer.reset();
        self.timer.unpause();
    }
}

impl Default for HitFlash {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
        timer.pause();
        Self { timer, flash_colour: Colour::RED, original_colour: None }
    }
}

#[derive(Component)]
pub struct FloatingText {
    pub ttl: Timer,
    pub rise_distance: f32,
}

impl Default for FloatingText {
    fn default() -> Self {
        Self { ttl: Timer::from_seconds(1.0, TimerMode::Once), rise_distance: 10.0 }
    }
}

#[derive(Component, Copy, Clone)]
pub struct Damage {
    pub amount: i32,
    pub is_crit: bool,
}
