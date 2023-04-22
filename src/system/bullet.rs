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
      commands.entity(entity).insert(ShouldDespawn);
    }
  }
}

pub fn bullet_collision_system(
  mut commands: Commands,
  mut query: Query<(&mut Collider, &Transform, Entity, &Owner), (With<Bullet>, With<Collider>, With<Owner>)>,
  mut potential_query: Query<(&Collider, &Transform, &mut Health, Entity), (Without<Bullet>, With<Collider>, With<Health>)>
) {
  for (collider, transform, entity, owner) in &mut query {
    for (potential_collider, potential_transform, mut potential_health, potential_entity) in &mut potential_query {
      if potential_entity == owner.0 { // Source of bullet cannot be hit
        continue;
      }
      if transform.translation.distance(potential_transform.translation) <= collider.radius + potential_collider.radius {
        potential_health.take_damage(1);
        commands.entity(entity).insert(ShouldDespawn);
      }
    }
  }
}