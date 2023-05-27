use bevy::prelude::*;
use crate::{component::*, GameState, resource::PlayerLevel};

pub fn player_control(
  mouse_button_input: Res<Input<MouseButton>>,
  windows: Query<&Window>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut query: Query<(&IsPlayer, &mut Engine), (With<IsPlayer>, With<Engine>)>,
) {
  for (_, mut engine) in &mut query {
      if mouse_button_input.pressed(MouseButton::Left) {
          // Calculate current position to mouse position
          let (camera, camera_transform) = camera_q.single();
          let window = windows.get_single().expect("no primary window");

          engine.target = window.cursor_position()
              .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
              .map(|ray| ray.origin.truncate());
      } else {
          engine.target = None;
      }
  }
}

pub fn pause_control(
  key_input: Res<Input<KeyCode>>,
  game_state: Res<State<GameState>>,
  mut change_game_state: ResMut<NextState<GameState>>,
  mut query: Query<&mut CameraShake>,
) {
  if key_input.just_pressed(KeyCode::Escape) {
    match game_state.0 {
      GameState::Running => change_game_state.set(GameState::Paused),
      GameState::Paused => change_game_state.set(GameState::Running),
      _ => ()
    }
  }

  // Debug camera shake
  if key_input.just_pressed(KeyCode::R) {
    for mut shake in &mut query {
      shake.trauma = 5.0;
    }
  }
}

pub fn level_up_system(
  mut level: ResMut<PlayerLevel>,
  mut query: Query<&mut Cargo, With<IsPlayer>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for mut cargo in &mut query {
    if cargo.0 >= level.required_cargo_to_level() {
      cargo.0 -= level.required_cargo_to_level();
      level.value += 1;
      next_state.set(GameState::Selection);
    }
  }
}