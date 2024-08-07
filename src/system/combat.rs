use crate::{
    component::*,
    resource::{Fonts, TakeDamageEvent},
    util::{Colour, RenderLayer},
};
use bevy::prelude::*;

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Health, Entity), Without<ShouldDespawn>>,
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
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut take_damage_events: EventReader<TakeDamageEvent>,
    mut query: Query<(
        &Transform,
        &mut Health,
        Option<&IsPlayer>,
        Option<&mut HitFlash>,
    )>,
    mut camera: Query<&mut CameraShake>,
) {
    for ev in take_damage_events.read() {
        if let Ok((transform, mut health, is_player, hit_flash)) = query.get_mut(ev.entity) {
            health.take_damage(ev.damage.amount);

            if is_player.is_some() {
                if let Ok(mut shake) = camera.get_single_mut() {
                    shake.trauma = ev.damage.amount.clamp(0, 5) as f32;
                }
            } else {
                // Floating Text
                commands.spawn((
                    FloatingText::default(),
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{}", ev.damage.amount),
                            TextStyle {
                                font: fonts.primary.clone(),
                                font_size: if ev.damage.is_crit { 14.0 } else { 12.0 },
                                color: if ev.damage.is_crit { Colour::YELLOW } else { Colour::WHITE },
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y + 10.0,
                            RenderLayer::Effects.as_z(),
                        ),
                        ..default()
                    },
                ));
            }

            if let Some(mut hit_flash) = hit_flash {
                hit_flash.hit();
            }
            
        }
    }
}
