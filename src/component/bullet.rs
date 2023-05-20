use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub ttl: Timer,
}

impl Bullet {

    pub fn new(seconds_to_live: f32) -> Bullet {
      Bullet { ttl: Timer::from_seconds(seconds_to_live, TimerMode::Once) }
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