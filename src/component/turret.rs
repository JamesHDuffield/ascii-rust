use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::DisplayName;

#[derive(Component)]
pub struct Range {
    pub max: f32,
}

#[derive(Component)]
pub struct FireRate {
    pub rate: f32,
    pub timer: Timer,
}

impl FireRate {
    fn from_rate_in_seconds(rate: f32) -> FireRate {
        FireRate { rate, timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating) }
    }
}

#[derive(Component)]
pub struct Targets {
    pub target: Option<Entity>,
}

impl Targets {
    fn default() -> Targets {
        Targets { target: None }
    }
}

#[derive(Component)]
pub enum TurretClass {
    AutoCannon,
    BlastLaser,
    RocketLauncher,
    MineLauncher,
}

#[derive(Bundle)]
pub struct TurretBundle {
    name: DisplayName,
    range: Range,
    fire_rate: FireRate,
    target: Targets,
    class: TurretClass,
}

impl TurretBundle {

    pub fn auto_cannon() -> TurretBundle {
        TurretBundle {
            name: DisplayName(String::from("Auto Cannon")),
            class: TurretClass::AutoCannon,
            range: Range { max: 200.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(1.0),
        }
    }

    pub fn blast_laser() -> TurretBundle {
        TurretBundle {
            name: DisplayName(String::from("Blast Laser")),
            class: TurretClass::BlastLaser,
            range: Range { max: 200.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(2.0),
        }
    }

    pub fn rocket_launcher() -> TurretBundle {
        TurretBundle {
            name: DisplayName(String::from("Rocket Launcher")),
            class: TurretClass::RocketLauncher,
            range: Range { max: 800.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(0.5),
        }
    }

    pub fn mine_launcher() -> TurretBundle {
        TurretBundle {
            name: DisplayName(String::from("Mine Launcher")),
            class: TurretClass::MineLauncher,
            range: Range { max: 800.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(0.9),
        }
    }

    pub fn random_starting_weapon() -> TurretBundle {
        let starting_weapons = vec![|| TurretBundle::auto_cannon(), || TurretBundle::blast_laser(), || TurretBundle::rocket_launcher(), || TurretBundle::mine_launcher()];
        starting_weapons.choose(&mut rand::thread_rng()).unwrap()()
    }

}