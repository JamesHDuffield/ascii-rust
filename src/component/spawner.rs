use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Spawner {
    pub cooldown_timer: Timer,
}

impl Spawner {

  pub fn new(seconds: f32, inital_delay_seconds: f32) -> Spawner {
    let mut timer = Timer::from_seconds(seconds, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs_f32(seconds - inital_delay_seconds));
    Spawner { cooldown_timer: timer }
  }

}