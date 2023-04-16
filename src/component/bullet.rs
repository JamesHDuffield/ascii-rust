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