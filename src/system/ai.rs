use bevy::prelude::*;
use crate::component::*;

pub fn ai_system(
  mut query: Query<&mut Engine, (With<AI>, With<Transform>, With<Engine>)>,
  player_query: Query<&Transform, (With<IsPlayer>, With<Transform>, Without<AI>)>,
) {
  if let Ok(player_transform) = player_query.get_single() {
    for mut engine in &mut query {
      // Orbit the player
      engine.method = EngineMethod::Orbit(100.0);
      engine.target = Some(player_transform.translation.truncate());
    }
  }
}