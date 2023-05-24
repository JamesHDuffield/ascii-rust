use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, util::*};

use super::TurretFireEvent;

pub fn fire_pierce_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectSize)>,
    parent_query: Query<(&Transform, &WillTarget)>,
    target_query: Query<&Transform>,
    mut potential_query: Query<(Entity, &Transform, &Targettable, &Collider, &mut Health)>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::PierceLaser => {

                // Get Turret Info
                let Ok((parent, targets, damage, size)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Target Info
                let Ok(target_transform) = target_query.get(target) else { continue; };

                // Get Parent Info
                let Ok((parent_transform, parent_will_target)) = parent_query.get(parent.get()) else { continue; };

                // Spawn graphic
                const LASER_LENGTH: f32 = 8000.0;
                let origin = parent_transform.translation.truncate();
                let target = target_transform.translation.truncate();
                let end = (target - origin).normalize() * LASER_LENGTH;
                commands.spawn((
                    Bullet::new(1.0),
                    LaserRender,
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Line(origin, end)),
                        transform: Transform::from_xyz(0., 0., RenderLayer::Bullet.as_z()),
                        ..default()
                    },
                    Stroke::new(Colour::YELLOW, size.0),
                    Owner(parent.get()),
                    DespawnWithScene,
                ));

                // Hit everything on the path
                potential_query
                    .iter_mut()
                    .filter(|a| a.0 != parent.get() && parent_will_target.0.contains(&a.2.0))
                    .filter(|a| Math::distance_from_point_to_line(a.1.translation.truncate(), origin, end) <= a.3.radius + size.0)
                    .for_each(|mut hit| hit.4.take_damage(damage.amount));

            },
            _ => (),
        }
    }
}