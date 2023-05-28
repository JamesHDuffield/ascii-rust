use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_needler(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::Needler => {

                // Get Turret Info
                let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn needle
                let origin = parent_transform.translation.truncate();
                for _ in 0..shots.amount {
                    commands.spawn((
                        Bullet::new(3.0),
                        Text2dBundle {
                            text: Text::from_section(
                                "-",
                                TextStyle {
                                    font: fonts.primary.clone(),
                                    font_size: 12.0,
                                    color: colour.0,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            transform: Transform {
                                translation: origin.extend(RenderLayer::Bullet.as_z()),
                                ..Default::default()
                            },
                            ..default()
                        },
                        Physics::default(),
                        Engine::new_with_steering(400.0, 5.0, 0.0),
                        Seeker(target),
                        Collider { radius: 5.0 },
                        Owner(parent.get()),
                        DirectDamage(damage.amount),
                        DespawnWithScene,
                    ));
                }

            },
            _ => (),
        }
    }
}