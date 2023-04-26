use bevy::prelude::*;
use crate::{component::*, GameState};

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
  mut change_game_state: ResMut<NextState<GameState>>
) {
  if key_input.just_pressed(KeyCode::Escape) {
    match game_state.0 {
      GameState::Running => change_game_state.set(GameState::Paused),
      GameState::Paused => change_game_state.set(GameState::Running),
      _ => ()
    }
  }
}