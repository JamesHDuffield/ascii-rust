use bevy::{prelude::*, ecs::query::QueryEntityError};
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, util::*};

use super::{TurretFireEvent, get_closest_target};


fn spawn_link(commands: &mut Commands, target_query: &mut Query<(&Transform, &mut Health)>, origin: Vec2, target: Entity, damage: &DoesDamage, jump: u8, colour: &EffectColour) -> Result<Vec2, QueryEntityError> {
    // Get Target Info
    let (target_transform, mut target_health) = target_query.get_mut(target)?;
    let target = target_transform.translation.truncate();
    // Spawn graphic
    commands.spawn((
        Bullet::new(0.2 + (jump as f32) * 0.1),
        LaserRender,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(origin, target)),
            transform: Transform::from_xyz(0., 0., RenderLayer::Bullet.as_z()),
            ..default()
        },
        Stroke::new(colour.0, 2.0),
        DespawnWithScene,
    ));
    // Immediate hit
    target_health.take_damage(damage.amount);
    Ok(target)
}

pub fn fire_chain_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<(&Transform, &WillTarget)>,
    mut target_query: Query<(&Transform, &mut Health)>,
    potential_query: Query<(Entity, &Transform, &Targettable), (With<Targettable>, With<Transform>)>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::ChainLaser => {

                // Get Turret Info
                let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Target
                let Some(target) = targets.target else { continue; };

                // Get Parent Info
                let Ok((parent_transform, parent_will_target)) = parent_query.get(parent.get()) else { continue; };

                // Get all possible targets
                let mut potential_targets: Vec<(Entity, &Transform, &Targettable)> = potential_query
                    .iter()
                    .filter(|a| a.0 != parent.get() && parent_will_target.0.contains(&a.2.0))
                    .collect();

                // Get other nearby targets to bounce to
                let mut num_jumps = 0;
                let mut current_target = Some(target);
                let mut previous_position = parent_transform.translation.truncate();
                
                while num_jumps < shots.amount && current_target.is_some() {
                    num_jumps += 1;

                    
                    let Some(target) = current_target else { break; };
                    
                    // Remove target from potentials list so no repeats
                    potential_targets.retain(|potential| potential.0 != target);
                    
                    let result = spawn_link(&mut commands, &mut target_query, previous_position, target, damage, num_jumps, colour);

                    match result {
                        Ok(pos) => {
                            previous_position = pos;
                        }
                        Err(_) => break
                    }

                    current_target = get_closest_target(&mut potential_targets, previous_position)
                };

            },
            _ => (),
        }
    }
}