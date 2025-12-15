use bevy::prelude::*;
use crate::component::*;

pub fn rotator_system(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &Rotator)>,
) {
  for (mut transform, rotator) in &mut query {
    transform.rotate(Quat::from_rotation_z(rotator.speed * time.delta_secs()));
  }
}