use crate::{component::*, resource::TakeDamageEvent};
use bevy::prelude::*;

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Health, Entity), With<Health>>,
) {
    for (mut health, entity) in &mut query {
        if health.health <= 0 {
            commands.entity(entity).insert(ShouldDespawn);
            continue;
        }

        // Recharge shield
        health.shield_recharge_cooldown.tick(time.delta());
        if health.shield_recharge_cooldown.finished() {
            health.shield_recharge_timer.tick(time.delta());
            if health.shield_recharge_timer.just_finished() {
                if health.shield == health.max_shield {
                    return;
                }
                health.shield += 1;
            }
        }
    }
}

pub fn take_damage_events(
    mut take_damage_events: EventReader<TakeDamageEvent>,
    mut query: Query<(&mut Health, Option<&IsPlayer>)>,
    mut camera: Query<&mut CameraShake>,
) {
    for ev in take_damage_events.iter() {
        if let Ok((mut health, is_player)) = query.get_mut(ev.entity) {
            health.take_damage(ev.amount);
            if is_player.is_some() {
                if let Ok(mut shake) = camera.get_single_mut() {
                    shake.trauma = 5.0;
                }
            }
        } 
    }
}
