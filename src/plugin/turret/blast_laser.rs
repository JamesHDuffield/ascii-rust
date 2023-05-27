use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, util::*, resource::TakeDamageEvent};

use super::TurretFireEvent;

pub fn fire_blast_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::BlastLaser => {

                // Get Turret Info
                let Ok((parent, targets, damage, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Target Info
                let Ok(target_transform) = target_query.get(target) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn graphic
                let origin = parent_transform.translation.truncate();
                let target_pos = target_transform.translation.truncate();
                commands.spawn((
                    Bullet::new(0.1),
                    LaserRender,
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Line(origin, target_pos)),
                        transform: Transform::from_xyz(0., 0., RenderLayer::Bullet.as_z()),
                        ..default()
                    },
                    Stroke::new(colour.0, 1.0),
                    Owner(parent.get()),
                    DespawnWithScene,
                ));

                // Immediate hit
                take_damage_event.send(TakeDamageEvent { entity: target, amount: damage.amount });

            },
            _ => (),
        }
    }
}