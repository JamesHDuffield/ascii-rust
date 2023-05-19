use bevy::prelude::*;

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

}