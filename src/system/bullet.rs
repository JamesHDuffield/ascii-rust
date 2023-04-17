use std::ops::BitAnd;

use bevy::prelude::*;
use crate::component::*;

pub fn bullet_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut Bullet, Entity), With<Bullet>>,
) {
  for (mut bullet, entity) in &mut query {
    bullet.ttl.tick(time.delta());
    if bullet.ttl.just_finished() {
      commands.entity(entity).despawn();
    }
  }
}

pub fn bullet_collision_system(
  mut commands: Commands,
  mut query: Query<(&mut Collider, &Transform, Entity), (With<Bullet>, With<Collider>)>,
  mut potential_query: Query<(&Collider, &Transform, &mut Health), (Without<Bullet>, With<Collider>, With<Health>)>
) {
  for (collider, transform, entity) in &mut query {
    for (potential_collider, potential_transform, mut potential_health) in &mut potential_query {
      if collider.mask.bitand(potential_collider.layer) == 0 {
        continue;
      }
      if transform.translation.distance(potential_transform.translation) <= collider.radius + potential_collider.radius {
        potential_health.take_damage(1);
        commands.entity(entity).despawn();
      }
    }
  }
}