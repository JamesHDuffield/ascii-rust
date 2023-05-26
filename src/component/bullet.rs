use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Bullet {
    pub ttl: Timer,
    pub max_hits_per_entity: u8,
    pub entities_hit: HashMap<Entity, u8>,
    pub despawn_on_hit: bool,
}

impl Bullet {
    pub fn new(seconds_to_live: f32) -> Bullet {
      Bullet { ttl: Timer::from_seconds(seconds_to_live, TimerMode::Once), ..Default::default() }
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self { ttl: Timer::from_seconds(1.0, TimerMode::Once), max_hits_per_entity: 1, entities_hit: HashMap::new(), despawn_on_hit: true }
    }
}

#[derive(Component)]
pub struct LaserRender;

#[derive(Component)]
pub struct Seeker(pub Entity);

#[derive(Component)]
pub struct DirectDamage(pub i32);

#[derive(Component)]
pub struct AoeDamage {
    pub damage: i32,
    pub range: f32,
}