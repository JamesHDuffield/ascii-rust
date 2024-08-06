use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_auto_cannon(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.read() {
        match ev.class {
            TurretClass::AutoCannon => {

                // Get Turret Info
                let Ok((parent, targets, damage, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Target Info
                let Ok(target_transform) = target_query.get(target) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn bullet
                let bullet_speed = 1000.0;
                let origin = parent_transform.translation.truncate();
                let destination = target_transform.translation.truncate();
                let direction = (destination - origin).normalize();
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
                        velocity: direction * bullet_speed,
                        ..Default::default()
                    },
                    Collider { radius: 5.0 },
                    Owner(parent.get()),
                    DirectDamage(damage.roll()),
                    DespawnWithScene,
                ));

            },
            _ => (),
        }
    }
}