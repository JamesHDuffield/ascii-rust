use bevy::prelude::*;

use crate::component::*;

pub fn expanding_collider_system(
  mut query: Query<(&mut Collider, &ExpandingCollider, &Bullet)>,
) {
  for (mut collider, expanding, bullet) in &mut query {
    collider.radius = bullet.ttl.fraction() * expanding.final_radius;
  }
}