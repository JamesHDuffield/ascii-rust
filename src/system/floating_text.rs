use bevy::prelude::*;

use crate::component::FloatingText;

pub fn floating_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingText, &mut TextColor)>,
) {
    for (entity, mut transform, mut floating_text, mut text) in &mut query {
        floating_text.ttl.tick(time.delta());

        transform.translation.y +=  time.delta().as_secs_f32() / floating_text.ttl.duration().as_secs_f32() * floating_text.rise_distance;
        text.0.set_alpha(floating_text.ttl.fraction_remaining());

        if floating_text.ttl.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}