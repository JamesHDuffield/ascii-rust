use crate::{component::*, resource::TakeDamageEvent};
use bevy::prelude::*;

pub fn bullet_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Bullet, Entity), With<Bullet>>,
) {
    for (mut bullet, entity) in &mut query {
        bullet.ttl.tick(time.delta());
        if bullet.ttl.just_finished() {
            commands.entity(entity).insert(ShouldDespawn);
        }
    }
}

pub fn bullet_collision_system(
    mut commands: Commands,
    mut query: Query<
        (
            &Collider,
            &Transform,
            Entity,
            &Owner,
            Option<&DirectDamage>,
            Option<&AoeDamage>,
            &mut Bullet,
        ),
        (With<Bullet>, With<Collider>, With<Owner>),
    >,
    potential_query: Query<
        (&Collider, &Transform, Entity),
        (Without<Bullet>, With<Collider>, With<Health>),
    >,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for (collider, transform, entity, owner, direct_damage, aoe_damage, mut bullet) in &mut query {
        // Get all potentials
        let potentials = potential_query
            .iter()
            .filter_map(|potential| {
                if potential.2 == owner.0 {
                    // Source of bullet cannot be hit
                    return None;
                }
                Some(potential)
            })
            .collect::<Vec<_>>();

        // Sort by distance to bullet
        let hit = potentials.iter().find(|potential| {
            transform
                .translation
                .truncate()
                .distance(potential.1.translation.truncate())
                <= collider.radius + potential.0.radius
                && bullet.entities_hit.get(&potential.2).unwrap_or(&0) < &bullet.max_hits_per_entity
        });

        if let Some((_collider, _transform, potential_entity)) = hit {
            
            if let Some(direct_damage) = direct_damage {
                
                let number_of_times_hit = bullet.entities_hit.entry(*potential_entity).or_insert(0);
                *number_of_times_hit += 1;

                take_damage_event.send(TakeDamageEvent { entity: *potential_entity, damage: direct_damage.0 });

            }

            if let Some(aoe_damage) = aoe_damage {
                let all_hits: Vec<_> = potentials
                    .iter()
                    .filter(|potential| {
                        transform
                            .translation
                            .truncate()
                            .distance(potential.1.translation.truncate())
                            <= aoe_damage.range + potential.0.radius
                            && bullet.entities_hit.get(&potential.2).unwrap_or(&0)
                                < &bullet.max_hits_per_entity
                    })
                    .collect();
                for h in all_hits.iter() {
                    let number_of_times_hit = bullet.entities_hit.entry(h.2).or_insert(0);
                    *number_of_times_hit += 1;

                    take_damage_event.send(TakeDamageEvent { entity: h.2, damage: aoe_damage.damage });
                }
            }
            if bullet.despawn_on_hit {
                commands.entity(entity).insert(ShouldDespawn);
            }
            break;
        }
    }
}
