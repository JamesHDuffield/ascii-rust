use bevy::prelude::*;
use crate::component::*;

pub fn camera_follow(
  time: Res<Time>,
  player_q: Query<&Transform, (With<Transform>, With<IsPlayer>, Without<MainCamera>)>,
  mut camera_q: Query<&mut Transform, (With<Transform>, With<MainCamera>, Without<IsPlayer>)>,
) {

  if let Ok(mut camera_transform) = camera_q.get_single_mut() {

    if let Ok(player_transform) = player_q.get_single() {

      // Calculate the new camera position based on the player's position
      let target_position = Vec3::new(
          player_transform.translation.x + 1.0,
          player_transform.translation.y,
          camera_transform.translation.z, // Keep the original camera z position
      );

      // Smoothly interpolate the camera position towards the target position
      camera_transform.translation = camera_transform.translation.lerp(target_position, 5.0 * time.delta_seconds());
    }

  }

}