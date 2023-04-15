use bevy::prelude::*;
use crate::component::*;

pub fn engine_system(
  time: Res<Time>,
  mut query: Query<(&Transform, &mut Physics, &mut Engine), (With<Transform>, With<Physics>, With<Engine>)>,
) {
  for (transform, mut physics, mut engine) in &mut query {
      let current = transform.translation.truncate();
      if let Some(target) = engine.target {
          engine.speed += engine.power * time.delta_seconds();
          if engine.speed > engine.max_speed { engine.speed = engine.max_speed; }
          let to_target = (target - current).normalize() * engine.speed;
          physics.add_force(to_target);
      } else {
          engine.speed -= engine.power * time.delta_seconds() * engine.depower_factor;
          if engine.speed < 0.0 { engine.speed = 0.0 }
      }
  }
}