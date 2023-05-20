use crate::component::*;
use bevy::prelude::*;

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Health, Entity, Option<&Upgrades>), With<Health>>,
) {
    for (mut health, entity, upgrades) in &mut query {
        if health.health <= 0 {
            commands.entity(entity).insert(ShouldDespawn);
            continue;
        }

        // Recharge shield
        let shield_cooldown_modifier = upgrades.map_or(1.0, |up| 1.0 + up.shield_cooldown as f32 * 1.0);
        health.shield_recharge_cooldown.tick(time.delta().mul_f32(shield_cooldown_modifier));
        if health.shield_recharge_cooldown.finished() {
            let shield_recharge_modifier = upgrades.map_or(1.0, |up| 1.0 + up.shield_recharge as f32 * 1.0);
            health.shield_recharge_timer.tick(time.delta().mul_f32(shield_recharge_modifier));
            if health.shield_recharge_timer.just_finished() {
                if health.shield == health.max_shield {
                    return;
                }
                health.shield += 1;
            }
        }
    }
}
