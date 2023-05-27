use crate::{component::*, util::Math};
use bevy::prelude::*;
use bevy_parallax::ParallaxMoveEvent;

pub fn camera_follow(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Transform>, With<IsPlayer>, Without<MainCamera>)>,
    mut camera_q: Query<
        (&Transform, &mut CameraShake),
        (With<Transform>, With<MainCamera>, Without<IsPlayer>),
    >,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok((camera_transform, mut shake)) = camera_q.get_single_mut() {
        if let Ok(player_transform) = player_q.get_single() {
            // Calculate the new camera position based on the player's position
            let target_position = Vec2::new(
                player_transform.translation.x + 1.0,
                player_transform.translation.y,
            );

            let current_position = camera_transform.translation.truncate();

            let smooth_move_position = current_position
                .lerp(target_position, 5.0 * time.delta_seconds())
                + shake.trauma * Math::random_2d_unit_vector();

            shake.trauma = f32::max(shake.trauma - shake.decay * time.delta_seconds(), 0.0);

            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: smooth_move_position - current_position,
            });
        }
    }
}
