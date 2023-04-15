use bevy::prelude::*;

pub fn quaternion_from_2d_vector(vector: Vec2) -> Quat {
  let angle = vector.y.atan2(vector.x);
  Quat::from_rotation_z(angle)
}