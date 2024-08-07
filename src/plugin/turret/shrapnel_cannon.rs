use std::f32::consts::PI;
use rand::Rng;

use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_shrapnel_cannon(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.read() {
        match ev.class {
            TurretClass::ShrapnelCannon => {

                // Get Turret Info
                let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Target Info
                let Ok(target_transform) = target_query.get(target) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn bullets
                const SPREAD: f32 = PI / 4.0;
                const SPEED_VARIANCE: f32 = 400.0;

                let bullet_speed = 600.0;
                let origin = parent_transform.translation.truncate();
                let destination = target_transform.translation.truncate();
                let direction = (destination - origin).normalize();

                let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                for _ in 0..shots.amount {
                    let random_angle = rng.gen_range(-SPREAD / 2.0..SPREAD / 2.0);
                    let spread_direction = Vec2::from_angle(random_angle).rotate(direction);
                    let random_speed = rng.gen_range(-SPEED_VARIANCE / 2.0..SPEED_VARIANCE / 2.0) + bullet_speed;
                    commands.spawn((
                        Bullet::new(1.2),
                        Text2dBundle {
                            text: Text::from_section(
                                ".",
                                TextStyle {
                                    font: fonts.primary.clone(),
                                    font_size: 16.0,
                                    color: colour.0,
                                },
                            )
                            .with_justify(JustifyText::Center),
                            transform: Transform {
                                translation: origin.extend(RenderLayer::Bullet.as_z()),
                                ..Default::default()
                            },
                            ..default()
                        },
                        Physics {
                            velocity: spread_direction * random_speed,
                            ..Default::default()
                        },
                        Collider { radius: 5.0 },
                        Owner(parent.get()),
                        DirectDamage(damage.roll()),
                        DespawnWithScene,
                    ));
                }
            },
            _ => (),
        }
    }
}