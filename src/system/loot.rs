use bevy::prelude::*;
use crate::component::*;

pub fn loot_magnet_system(
  query: Query<(&Magnet, &Transform), (With<Magnet>, With<Transform>)>,
  mut loot_query: Query<(&mut Physics, &Transform), (With<IsLoot>, With<Physics>, With<Transform>, Without<Magnet>)>
) {
  for (magnet, transform) in &query {
    for (mut physics, loot_transform) in &mut loot_query {
      if loot_transform.translation.distance(transform.translation) > magnet.range {
        continue;
      }
      let direction = (transform.translation - loot_transform.translation).normalize_or_zero().truncate();
      physics.add_force(direction * magnet.strength);
    }
  }
}