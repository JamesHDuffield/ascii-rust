use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_rocket_launcher(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.read() {
        match ev.class {
            TurretClass::RocketLauncher => {

                // Get Turret Info
                let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn rocket
                let origin = parent_transform.translation.truncate();
                for _ in 0..shots.amount {
                    commands.spawn((
                        Bullet::new(3.0),
                        GlyphBundle::new("!", colour.0, 12.0, fonts.primary.clone()),
                        Transform {
                            translation: origin.extend(RenderLayer::Bullet.as_z()),
                            ..Default::default()
                        },
                        BaseGlyphRotation {
                            rotation: Quat::from_rotation_z(PI / 2.0),
                        },
                        Physics {
                            velocity: Math::random_2d_unit_vector() * 100.0,
                            ..Default::default()
                        },
                        Engine::new_with_steering(40.0, 10.0, 0.5),
                        Seeker(target),
                        Collider { radius: 5.0 },
                        Owner(parent.get()),
                        ExplodesOnDespawn {
                            colour: colour.0,
                            ..Default::default()
                        },
                        AoeDamage { damage: damage.roll(), range: 40.0 },
                        DespawnWithScene,
                    ));
                }

            },
            _ => (),
        }
    }
}