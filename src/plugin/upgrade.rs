use bevy::{prelude::*, utils::HashMap};
use std::{fmt::Display, time::Duration};

use crate::{
    component::{
        DoesDamage, EffectSize, Engine, FireRate, Health, IsPlayer, Magnet, MultiShot,
        TurretBundle, TurretClass,
    },
    AppState,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Resource)]
pub struct PlayerUpgrades(pub HashMap<UpgradeEvent, u8>);

impl PlayerUpgrades {
    pub fn display_for_ui(&self) -> Vec<String> {
        self.0
            .iter()
            .filter(|(_, level)| **level > 0)
            .map(|(upgrade, level)| format!("{:0>2} {:>16}", level, upgrade))
            .collect()
    }

    pub fn max_allowed_level() -> u8 {
        8
    }

    pub fn reached_max_passives(&self) -> bool {
        self.0
            .iter()
            .filter(|(upgrade, _)| match upgrade {
                UpgradeEvent::Passive(_) => true,
                _ => false,
            })
            .count()
            >= 6
    }

    pub fn reached_max_weapons(&self) -> bool {
        self.0
            .iter()
            .filter(|(upgrade, _)| match upgrade {
                UpgradeEvent::Weapon(_) => true,
                _ => false,
            })
            .count()
            >= 4
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum UpgradeEvent {
    Weapon(TurretClass),
    Passive(Passive),
}

impl Distribution<UpgradeEvent> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UpgradeEvent {
        match rng.gen_range(0..2) {
            0 => UpgradeEvent::Weapon(rand::random()),
            _ => UpgradeEvent::Passive(rand::random()),
        }
    }
}

impl Display for UpgradeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeEvent::Weapon(weapon) => write!(f, "{}", weapon),
            UpgradeEvent::Passive(passive) => write!(f, "{}", passive),
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Passive {
    Speed,
    Magnet,
    ShieldRecharge,
    ShieldCooldown,
}

impl Display for Passive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Passive::Speed => write!(f, "Speed"),
            Passive::Magnet => write!(f, "Magnet"),
            Passive::ShieldRecharge => write!(f, "Shield Boost"),
            Passive::ShieldCooldown => write!(f, "Quick Shield"),
        }
    }
}

impl Distribution<Passive> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Passive {
        match rng.gen_range(0..4) {
            0 => Passive::Speed,
            1 => Passive::ShieldRecharge,
            2 => Passive::ShieldCooldown,
            _ => Passive::Magnet,
        }
    }
}

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerUpgrades(HashMap::new()))
            .add_event::<UpgradeEvent>()
            .add_systems(
                (
                    record_upgrade,
                    upgrade_weapon_event,
                    upgrade_magnet_event,
                    upgrade_speed_event,
                    upgrade_shield_events,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}

fn record_upgrade(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut player_upgrades: ResMut<PlayerUpgrades>,
) {
    for ev in upgrade_event.iter() {
        let level = player_upgrades.0.entry(*ev).or_insert(0);
        *level += 1;
    }
}

fn upgrade_weapon_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut commands: Commands,
    player_query: Query<(Entity, Option<&Children>), With<IsPlayer>>,
    turret_query: Query<&TurretClass>,
    mut existing_auto_cannon: Query<&mut FireRate>,
    mut existing_rocket_launcher: Query<&mut MultiShot>,
    mut existing_shrapnel_cannon: Query<&mut DoesDamage>,
    mut existing_mine_launcher: Query<&mut EffectSize>,
) {
    for ev in upgrade_event.iter() {
        match ev {
            UpgradeEvent::Weapon(weapon) => {
                // Get player
                for (player_entity, children) in &player_query {
                    // Search for existing
                    let existing = match children {
                        Some(children) => children.iter().find(|child| {
                            if let Ok(turret) = turret_query.get(**child) {
                                return turret == weapon;
                            }
                            return false;
                        }),
                        None => None,
                    };

                    match existing {
                        Some(entity) => {
                            // TODO split up logic into systems
                            match weapon {
                                TurretClass::AutoCannon => {
                                    let mut fire_rate =
                                        existing_auto_cannon.get_mut(*entity).unwrap();
                                    let new_rate = fire_rate.rate * 2.0;
                                    fire_rate.set_rate_in_seconds(new_rate);
                                }
                                TurretClass::BlastLaser => {
                                    let mut fire_rate =
                                        existing_auto_cannon.get_mut(*entity).unwrap();
                                    let new_rate = fire_rate.rate * 2.0;
                                    fire_rate.set_rate_in_seconds(new_rate);
                                }
                                TurretClass::RocketLauncher => {
                                    let mut shots =
                                        existing_rocket_launcher.get_mut(*entity).unwrap();
                                    shots.amount += 1;
                                }
                                TurretClass::ShrapnelCannon => {
                                    let mut damage =
                                        existing_shrapnel_cannon.get_mut(*entity).unwrap();
                                    damage.amount += 1;
                                }
                                TurretClass::MineLauncher => {
                                    let mut size = existing_mine_launcher.get_mut(*entity).unwrap();
                                    size.0 *= 1.5;
                                },
                                TurretClass::ChainLaser => {
                                    let mut shots =
                                        existing_rocket_launcher.get_mut(*entity).unwrap();
                                    shots.amount += 1;
                                }
                            }
                        }
                        None => {
                            commands.entity(player_entity).with_children(|parent| {
                                parent.spawn(TurretBundle::from_class(weapon));
                            });
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn upgrade_magnet_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Magnet, With<IsPlayer>>,
) {
    for ev in upgrade_event.iter() {
        match ev {
            UpgradeEvent::Passive(Passive::Magnet) => {
                for mut magnet in &mut query {
                    magnet.range += 200.0;
                    magnet.strength += 2.0;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_speed_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Engine, With<IsPlayer>>,
) {
    for ev in upgrade_event.iter() {
        match ev {
            UpgradeEvent::Passive(Passive::Speed) => {
                for mut engine in &mut query {
                    engine.power += 2.0;
                    engine.max_speed += 4.0;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_shield_events(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Health, With<IsPlayer>>,
) {
    for ev in upgrade_event.iter() {
        match ev {
            UpgradeEvent::Passive(Passive::ShieldRecharge) => {
                for mut health in &mut query {
                    let mut new_timer = health.shield_recharge_timer.duration().as_secs_f32() - 0.5;
                    if new_timer < 0.1 {
                        new_timer = 0.1;
                    }
                    health
                        .shield_recharge_timer
                        .set_duration(Duration::from_secs_f32(new_timer));
                }
            }
            UpgradeEvent::Passive(Passive::ShieldCooldown) => {
                for mut health in &mut query {
                    let mut new_timer =
                        health.shield_recharge_cooldown.duration().as_secs_f32() - 1.0;
                    if new_timer < 0.5 {
                        new_timer = 0.5;
                    }
                    health
                        .shield_recharge_cooldown
                        .set_duration(Duration::from_secs_f32(new_timer));
                }
            }
            _ => (),
        }
    }
}
