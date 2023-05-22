use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, util::*};

use super::TurretFireEvent;

pub fn fire_blast_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage)>,
    parent_query: Query<&Transform>,
    mut target_query: Query<(&Transform, &mut Health)>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::BlastLaser => {

                // Get Turret Info
                let Ok((parent, targets, damage)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Target Info
                let Ok((target_transform, mut target_health)) = target_query.get_mut(target) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn graphic
                let origin = parent_transform.translation.truncate();
                let target = target_transform.translation.truncate();
                commands.spawn((
                    Bullet::new(0.1),
                    LaserRender,
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Line(origin, target)),
                        transform: Transform::from_xyz(0., 0., RenderLayer::Bullet.as_z()),
                        ..default()
                    },
                    Stroke::new(Colour::RED, 1.0),
                    Owner(parent.get()),
                    DespawnWithScene,
                ));

                // Immediate hit
                target_health.take_damage(damage.amount);

            },
            _ => (),
        }
    }
}