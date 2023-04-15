use bevy::prelude::*;
use crate::component::*;
use crate::math;

pub fn physics_system(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &mut Physics, Option<&BaseGlyphRotation>), (With<Transform>, With<Physics>)>,
) {
  for (mut transform, mut physics, base_rotation) in &mut query {
      // Not sure how to avoid cloning here
      let current_acceleration = physics.acceleration.clone();
      let drag = physics.drag.clone();
      physics.velocity += current_acceleration;
      transform.translation += physics.velocity.extend(0.0) * time.delta_seconds();
      // TODO make acceleration ramp down
      physics.acceleration = Vec2::ZERO;
      physics.velocity *= 1.0 - (drag * time.delta_seconds());

      // Face velocity
      transform.rotation = math::quaternion_from_2d_vector(physics.velocity);

      if let Some(base_rotation) = base_rotation {
          transform.rotation *= base_rotation.rotation; // Multiplication is like combining rotations together
      }
  }
}