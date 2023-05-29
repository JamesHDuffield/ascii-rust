use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, util::*};

use super::TurretFireEvent;

pub fn fire_emp(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &DoesDamage, &EffectSize, &EffectColour)>,
    parent_query: Query<&Transform>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::Emp => {

                // Get Turret Info
                let Ok((parent, damage, size, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                
                let origin = parent_transform.translation.truncate();
                let time_to_live = 1.0;

                // Spawn graphic
                commands.spawn((
                    ExplosionRender {
                        origin,
                        radius: size.0,
                        ttl: Timer::from_seconds(time_to_live, TimerMode::Once),
                        fade_out: true,
                    },
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle {
                            center: origin,
                            radius: 0.0,
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, RenderLayer::Effects.as_z()),
                        ..default()
                    },
                    Stroke::new(colour.0, 1.0),
                ));

                // Spawn bullet that damages
                commands.spawn((
                    Bullet { ttl: Timer::from_seconds(time_to_live, TimerMode::Once), despawn_on_hit: false, ..Default::default() },
                    Transform::from_translation(parent_transform.translation),
                    Collider { radius: 0.0 },
                    ExpandingCollider { final_radius: size.0 },
                    DirectDamage(damage.roll()),
                    Owner(parent.get()),
                ));

            },
            _ => (),
        }
    }
}