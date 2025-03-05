use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use crate::{component::*, input::PlayerAction, resource::PlayerLevel, GameState};

pub fn player_control(
  windows: Query<&Window>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut query: Query<(&IsPlayer, &mut Engine, &ActionState<PlayerAction>), (With<IsPlayer>, With<Engine>, With<ActionState<PlayerAction>>)>,
) {
  for (_, mut engine, action_state) in &mut query {
      if action_state.pressed(&PlayerAction::Move) {
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
  key_input: Res<ButtonInput<KeyCode>>,
  game_state: Res<State<GameState>>,
  mut change_game_state: ResMut<NextState<GameState>>,
  mut query: Query<&mut CameraShake>,
) {
  if key_input.just_pressed(KeyCode::Escape) {
    match game_state.get() {
      GameState::Running => change_game_state.set(GameState::Paused),
      GameState::Paused => change_game_state.set(GameState::Running),
      _ => ()
    }
  }

  // Debug camera shake
  if key_input.just_pressed(KeyCode::KeyR) {
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
    if cargo.amount >= level.required_cargo_to_level() {
      cargo.amount -= level.required_cargo_to_level();
      level.value += 1;
      next_state.set(GameState::Selection);
    }
  }
}

pub fn zoom_control(
  key_input: Res<ButtonInput<KeyCode>>,
  mut camera_q: Query<
        &mut OrthographicProjection,
        (With<OrthographicProjection>, With<MainCamera>),
    >,
) {
  let scale_factor = 0.25;

  if key_input.just_pressed(KeyCode::NumpadAdd) {
    if let Ok(mut projection) = camera_q.get_single_mut() {
      projection.scale = (projection.scale - scale_factor).max(1.);
    }
  }

  if key_input.just_pressed(KeyCode::NumpadSubtract) {
    if let Ok(mut projection) = camera_q.get_single_mut() {
      projection.scale = (projection.scale + scale_factor).min(3.);
    }
  }
}