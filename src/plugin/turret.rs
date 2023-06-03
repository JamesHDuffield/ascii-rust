mod blast_laser;
mod rocket_launcher;
mod mine_launcher;
mod auto_cannon;
mod shrapnel_cannon;
mod chain_laser;
mod pierce_laser;
mod emp;

use bevy::prelude::*;

use crate::AppState;
use crate::component::*;
use crate::game_not_paused;

use self::blast_laser::*;
use self::rocket_launcher::*;
use self::mine_launcher::*;
use self::auto_cannon::*;
use self::shrapnel_cannon::*;
use self::chain_laser::*;
use self::pierce_laser::*;
use self::emp::*;

use super::SoundEffectEvent;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TurretFireEvent>()
            .add_systems(
                (
                    turret_targetting_system,
                    turret_fire_system,
                )
                    .distributive_run_if(game_not_paused)
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .add_systems(
                (

                    fire_blast_laser,
                    fire_rocket_launcher,
                    fire_mine_launcher,
                    fire_auto_cannon,
                    fire_shrapnel_cannon,
                    fire_chain_laser,
                    fire_pierce_laser,
                    fire_emp,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}

pub struct TurretFireEvent {
    pub class: TurretClass,
    pub turret: Entity,
}

pub fn get_closest_target(
    potentials: &mut Vec<(Entity, &Transform, &Targettable)>,
    point: Vec2,
) -> Option<Entity> {
    potentials.sort_by(|a, b| {
    a.1.translation.truncate()
        .distance(point)
        .partial_cmp(&b.1.translation.truncate().distance(point))
        .unwrap()
    });
    potentials
        .first()
        .map(|potential| potential.0)
}

fn turret_targetting_system(
    mut query: Query<(&mut Targets, &Parent, &Range)>,
    target_query: Query<(Entity, &Transform, &Targettable)>,
    parent_query: Query<(&Transform, Entity, &WillTarget)>,
) {
    
    for (mut targets, parent, range) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity, parent_will_target)) = parent_query.get(parent.get()) {
            if let Some(target) = targets.target {
                // Check still in range
                if let Ok(current_target) = target_query.get(target) {
                    if current_target.1.translation.truncate().distance(parent_transform.translation.truncate()) > range.max {
                        targets.target = None;
                    }
                }
            } else {
                // Look for a target
                let mut potentials_without_parent: Vec<(Entity, &Transform, &Targettable)> = target_query
                    .iter()
                    .filter(|a| a.0 != parent_entity && parent_will_target.0.contains(&a.2.0))
                    .filter(|a| a.1.translation.truncate().distance(parent_transform.translation.truncate()) <= range.max)
                    .collect();
                targets.target = get_closest_target(&mut potentials_without_parent, parent_transform.translation.truncate());
            }
        }
    }
}

fn turret_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut FireRate, &TurretClass, &mut Targets, Entity)>,
    mut fire_event: EventWriter<TurretFireEvent>,
    mut sound_effect_event: EventWriter<SoundEffectEvent>,
) {
    for (mut fire_rate, class, mut targets, entity) in &mut query {
        if let Some(target) = targets.target {
            // Check target still exists and if not clear it
            match commands.get_entity(target) {
                None => {
                    targets.target = None;
                    break;
                }
                Some(_) => (),
            }
            fire_rate.timer.tick(time.delta());
            if fire_rate.timer.just_finished() {
                // Fire!
                fire_event.send(TurretFireEvent { class: *class, turret: entity });
                // Potential Sound Effect
                sound_effect_event.send(SoundEffectEvent::WeaponFire(*class));
            }
        } else {
            fire_rate.timer.reset();
        }
    }
}
