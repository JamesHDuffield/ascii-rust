use crate::component::*;
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
            &mut Collider,
            &Transform,
            Entity,
            &Owner,
            Option<&DirectDamage>,
            Option<&AoeDamage>,
        ),
        (With<Bullet>, With<Collider>, With<Owner>),
    >,
    potential_query: Query<
        (&Collider, &Transform, Entity),
        (Without<Bullet>, With<Collider>, With<Health>),
    >,
    mut effected_query: Query<&mut Health, With<Health>>,
) {
    for (collider, transform, entity, owner, direct_damage, aoe_damage) in &mut query {
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
            transform.translation.truncate().distance(potential.1.translation.truncate())
                <= collider.radius + potential.0.radius
        });

        if let Some((_collider, _transform, potential_entity)) = hit {

            if let Some(direct_damage) = direct_damage {
              if let Ok(mut health) = effected_query.get_mut(*potential_entity) {
                health.take_damage(direct_damage.0);
              }
            }
            if let Some(aoe_damage) = aoe_damage {
                let all_hits = potentials.iter().filter(|potential| {
                  transform.translation.truncate().distance(potential.1.translation.truncate()) <= aoe_damage.range + potential.0.radius
                });
                for h in all_hits {
                  if let Ok(mut health) = effected_query.get_mut(h.2) {
                    health.take_damage(aoe_damage.damage);
                  }
                }
            }
            commands.entity(entity).insert(ShouldDespawn);
            break;
        }
    }
}
