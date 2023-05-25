use bevy::prelude::*;
use std::{fmt::Display, time::Duration};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Component, Default)]
pub struct Range {
    pub max: f32,
}

#[derive(Component, Default)]
pub struct FireRate {
    pub rate: f32,
    pub timer: Timer,
}

impl FireRate {
    pub fn from_rate_in_seconds(rate: f32) -> FireRate {
        FireRate { rate, timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating) }
    }

    pub fn set_rate_in_seconds(&mut self, rate: f32) {
        self.rate = rate;
        self.timer.set_duration(Duration::from_secs_f32(1.0 / rate));
    }
}

#[derive(Component, Default)]
pub struct Targets {
    pub target: Option<Entity>,
}

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq, Default)]
pub enum TurretClass {
    #[default]
    AutoCannon,
    BlastLaser,
    RocketLauncher,
    MineLauncher,
    ShrapnelCannon,
    ChainLaser,
    PierceLaser,
}

impl Display for TurretClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurretClass::AutoCannon => write!(f, "Auto Cannon"),
            TurretClass::BlastLaser => write!(f, "Blast Laser"),
            TurretClass::RocketLauncher => write!(f, "Rocket Launcher"),
            TurretClass::MineLauncher => write!(f, "Mine Launcher"),
            TurretClass::ShrapnelCannon => write!(f, "Shrapnel Cannon"),
            TurretClass::ChainLaser =>  write!(f, "Chain Laser"),
            TurretClass::PierceLaser =>  write!(f, "Pierce Laser"),
        }
    }
}

impl Distribution<TurretClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TurretClass {
        match rng.gen_range(0..7) {
            0 => TurretClass::BlastLaser,
            1 => TurretClass::RocketLauncher,
            2 => TurretClass::MineLauncher,
            3 => TurretClass::ShrapnelCannon,
            4 => TurretClass::ChainLaser,
            5 => TurretClass::PierceLaser,
            _ => TurretClass::AutoCannon,
        }
    }
}

#[derive(Component, Default)]
pub struct DoesDamage {
    pub amount: i32,
}

#[derive(Component)]
pub struct MultiShot {
    pub amount: u8,
}

#[derive(Component, Default)]
pub struct EffectSize(pub f32);

impl Default for MultiShot {
    fn default() -> Self { MultiShot { amount: 1 } }
}

#[derive(Bundle, Default)]
pub struct TurretBundle {
    pub range: Range,
    pub fire_rate: FireRate,
    pub target: Targets,
    pub class: TurretClass,
    pub damage: DoesDamage,
    pub shots: MultiShot,
    pub size: EffectSize,
}

impl TurretBundle {

    pub fn auto_cannon() -> TurretBundle {
        TurretBundle {
            class: TurretClass::AutoCannon,
            range: Range { max: 200.0 },
            fire_rate: FireRate::from_rate_in_seconds(1.0),
            damage: DoesDamage { amount: 2 },
            ..Default::default()
        }
    }

    pub fn blast_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::BlastLaser,
            range: Range { max: 200.0 },
            fire_rate: FireRate::from_rate_in_seconds(2.0),
            damage: DoesDamage { amount: 1 },
            ..Default::default()
        }
    }

    pub fn rocket_launcher() -> TurretBundle {
        TurretBundle {
            class: TurretClass::RocketLauncher,
            range: Range { max: 800.0 },
            fire_rate: FireRate::from_rate_in_seconds(0.5),
            damage: DoesDamage { amount: 5 },
            ..Default::default()
        }
    }

    pub fn mine_launcher() -> TurretBundle {
        TurretBundle {
            class: TurretClass::MineLauncher,
            range: Range { max: 800.0 },
            fire_rate: FireRate::from_rate_in_seconds(0.9),
            damage: DoesDamage { amount: 5 },
            size: EffectSize(40.0),
            ..Default::default()
        }
    }

    pub fn shrapnel_cannon() -> TurretBundle {
        TurretBundle {
            class: TurretClass::ShrapnelCannon,
            range: Range { max: 400.0 },
            fire_rate: FireRate::from_rate_in_seconds(0.25),
            damage: DoesDamage { amount: 2 },
            shots: MultiShot { amount: 16 },
            ..Default::default()
        }
    }

    pub fn chain_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::ChainLaser,
            range: Range { max: 400.0 },
            fire_rate: FireRate::from_rate_in_seconds(0.4),
            damage: DoesDamage { amount: 1 },
            shots: MultiShot { amount: 3 },
            ..Default::default()
        }
    }

    pub fn pierce_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::PierceLaser,
            range: Range { max: 1200.0 },
            fire_rate: FireRate::from_rate_in_seconds(0.15),
            damage: DoesDamage { amount: 8 },
            size: EffectSize(1.0),
            ..Default::default()
        }
    }

    pub fn from_class(class: &TurretClass) -> TurretBundle {
        match class {
            TurretClass::AutoCannon => TurretBundle::auto_cannon(),
            TurretClass::BlastLaser => TurretBundle::blast_laser(),
            TurretClass::RocketLauncher => TurretBundle::rocket_launcher(),
            TurretClass::MineLauncher => TurretBundle::mine_launcher(),
            TurretClass::ShrapnelCannon => TurretBundle::shrapnel_cannon(),
            TurretClass::ChainLaser => TurretBundle::chain_laser(),
            TurretClass::PierceLaser => TurretBundle::pierce_laser(),
        }
    }

}