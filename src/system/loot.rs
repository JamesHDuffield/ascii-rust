use bevy::prelude::*;
use crate::{component::*, resource::Points};

pub fn loot_magnet_system(
  query: Query<(&Magnet, &Transform), (With<Magnet>, With<Transform>)>,
  mut loot_query: Query<(&mut Physics, &Transform), (With<IsLoot>, With<Physics>, With<Transform>, Without<Magnet>)>
) {
  for (magnet, transform) in &query {
    for (mut physics, loot_transform) in &mut loot_query {
      if loot_transform.translation.truncate().distance(transform.translation.truncate()) > magnet.range {
        continue;
      }
      let direction = (transform.translation.truncate() - loot_transform.translation.truncate()).normalize_or_zero();
      physics.add_force(direction * magnet.strength);
    }
  }
}

pub fn loot_cargo_collision(
  mut commands: Commands,
  mut query: Query<(&mut Cargo, &Transform, &Collider), (With<Cargo>, With<Transform>, With<Collider>)>,
  loot_query: Query<(&Transform, Entity, &Collider, Option<&WorthPoints>), (With<IsLoot>, With<Transform>, With<Collider>, Without<Cargo>)>,
  mut points: ResMut<Points>,
) {
  
  for (mut cargo, transform, collider) in &mut query {
    for (loot_transform, loot_entity, loot_collider, worth_points) in &loot_query {
      if loot_transform.translation.truncate().distance(transform.translation.truncate()) <= loot_collider.radius + collider.radius {
        cargo.0 += 1;
        if let Some(mut subcommand) = commands.get_entity(loot_entity) {
          subcommand.despawn(); // Direct despawn because adding ShouldDespawn has issues
          if let Some(worth_points) = worth_points {
              points.value += worth_points.value;
          }
        }
      }
    }
  }
}