use std::f32::consts::PI;

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
          let to_target = match engine.method {
            EngineMethod::Approach => approach(current, target),
            EngineMethod::Keep(distance) => keep_at_distance(current, target, distance),
            EngineMethod::Orbit(distance) => orbit(current, target, distance),
          };
          // Only kick in engines when target is not really close
          const CLOSE_TOLERANCE: f32 = 20.0;
          if to_target.length() > CLOSE_TOLERANCE {
            physics.add_force(to_target.normalize() * engine.speed);
          }
      } else {
          engine.speed -= engine.power * time.delta_seconds() * engine.depower_factor;
          if engine.speed < 0.0 { engine.speed = 0.0 }
      }
  }
}

fn approach(current: Vec2, target: Vec2) -> Vec2 {
  target - current
}

fn keep_at_distance(current: Vec2, target: Vec2, distance: f32) -> Vec2 {
  let new_target = target + (current - target).normalize() * distance;
  approach(current, new_target)
}

fn orbit(current: Vec2, target: Vec2, distance: f32) -> Vec2 {
  const ORBIT_TOLERANCE: f32 = 20.0;
  let distance_and_tolerance = distance + ORBIT_TOLERANCE;
  let distance_from_centre = current.distance(target);
  let towards_target = approach(current, target);
  if (distance_from_centre - distance_and_tolerance).abs() > ORBIT_TOLERANCE {
    return keep_at_distance(current, target, distance)
  } else {
    // Circle around
    let tangental = Quat::from_rotation_z(PI / 2.0).mul_vec3(towards_target.extend(0.0));
    let new_target = current + tangental.truncate();
    return approach(current, new_target)
  }
}