use bevy::prelude::*;
use rand::Rng;
pub struct Math;

impl Math {
  pub fn quaternion_from_2d_vector(vector: Vec2) -> Quat {
    let angle = vector.y.atan2(vector.x);
    Quat::from_rotation_z(angle)
  }

  pub fn random_2d_unit_vector() -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2 {
        x: rng.gen_range(-1.0..1.0),
        y: rng.gen_range(-1.0..1.0),
    }
      .try_normalize()
      .unwrap_or(Vec2::ONE)
  }
}