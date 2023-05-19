use bevy::prelude::*;
use crate::component::*;

pub fn ai_system(
  mut query: Query<(&Transform, &mut Engine, Entity), (With<AI>, With<Transform>, With<Engine>)>,
  other_query: Query<(&Transform, &Physics, Entity), (With<AI>, With<Transform>, With<Physics>)>,
  player_query: Query<&Transform, (With<IsPlayer>, With<Transform>, Without<AI>)>,
) {
  const PROXIMITY_CUTOFF: f32 = 20.0;
  const LOOK_AHEAD: f32 = 10.0;
  if let Ok(player_transform) = player_query.get_single() {
    for (transform, mut engine, entity) in &mut query {

      let neighbours: Vec<Vec2> = other_query
        .iter()
        .filter(|other| other.2 != entity)
        .filter(|other| other.0.translation.truncate().distance(transform.translation.truncate()) < 50.0)
        .map(|other| other.0.translation.truncate())
        .collect();
      let to_target = player_transform.translation.truncate() - transform.translation.truncate();
      
      let target_direction = if to_target.length() < PROXIMITY_CUTOFF {
        Vec2::ZERO
      } else {
        to_target.normalize_or_zero()
      };
        
      let seperation_direction = seperation(transform.translation.truncate(), &neighbours);
      let direction = (target_direction + seperation_direction).normalize_or_zero();
      
      engine.method = EngineMethod::Approach;

      if direction.length() > 0.0 {
        engine.target = Some(transform.translation.truncate() + direction * LOOK_AHEAD);
      } else {
        engine.target = None;
      };
    }
  }
}

fn seperation(position: Vec2, neighbours: &Vec<Vec2>) -> Vec2 {
  if neighbours.is_empty() {
    return Vec2::ZERO;
  }
  let away: Vec2 = neighbours.iter().map(|neighbour| position - *neighbour).sum();
  away.normalize_or_zero()
}
