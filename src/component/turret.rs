use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::fmt::Display;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TurretClass {
    AutoCannon,
    BlastLaser,
    RocketLauncher,
    MineLauncher,
}

impl Display for TurretClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurretClass::AutoCannon => write!(f, "Auto Cannon"),
            TurretClass::BlastLaser => write!(f, "Blast Laser"),
            TurretClass::RocketLauncher => write!(f, "Rocket Launcher"),
            TurretClass::MineLauncher => write!(f, "Mine Launcher"),
        }
    }
}

impl Distribution<TurretClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TurretClass {
        match rng.gen_range(0..4) {
            0 => TurretClass::BlastLaser,
            1 => TurretClass::RocketLauncher,
            2 => TurretClass::MineLauncher,
            _ => TurretClass::AutoCannon,
        }
    }
}

#[derive(Bundle)]
pub struct TurretBundle {
    range: Range,
    fire_rate: FireRate,
    target: Targets,
    class: TurretClass,
}

impl TurretBundle {

    pub fn auto_cannon() -> TurretBundle {
        TurretBundle {
            class: TurretClass::AutoCannon,
            range: Range { max: 200.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(1.0),
        }
    }

    pub fn blast_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::BlastLaser,
            range: Range { max: 200.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(2.0),
        }
    }

    pub fn rocket_launcher() -> TurretBundle {
        TurretBundle {
            class: TurretClass::RocketLauncher,
            range: Range { max: 800.0 },
            target: Targets::default(),
            fire_rate: FireRate::from_rate_in_seconds(0.5),
        }
    }

    pub fn mine_launcher() -> TurretBundle {
        TurretBundle {
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