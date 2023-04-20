use bevy::prelude::*;
use crate::component::*;

pub fn combat_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut Health, Entity), With<Health>>,
) {
  for (mut health, entity) in &mut query {
    if health.health <= 0 {
      commands.entity(entity).despawn();
      continue;
    }
    // Recharge shield
    health.shield_recharge_cooldown.tick(time.delta());
    if health.shield_recharge_cooldown.finished() {
      health.shield_recharge_timer.tick(time.delta());
      if health.shield_recharge_timer.just_finished() {
        if health.shield == health.max_shield {
          return;
        }
        health.shield += 1;
      }
    }
  }
}