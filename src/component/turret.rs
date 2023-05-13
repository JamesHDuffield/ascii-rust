use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {
    pub rate_of_fire: f32,
    pub timer: Timer,
    pub range: f32,
    pub target: Option<Entity>,
    pub class: TurretClass,
}

impl Turret {

    pub fn auto_cannon() -> Turret {
        let rate_of_fire = 1.0;
        Turret { target: None, rate_of_fire, range: 200.0, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::AutoCannon }
    }

    pub fn blast_laser() -> Turret {
        let rate_of_fire = 2.0;
        Turret { target: None, rate_of_fire, range: 200.0, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::BlastLaser }
    }

    pub fn rocket_launcher() -> Turret {
        let rate_of_fire = 0.5;
        Turret { target: None, rate_of_fire, range: 800.0, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::RocketLauncher }
    }

    pub fn mine_launcher() -> Turret {
        let rate_of_fire = 0.9;
        Turret { target: None, rate_of_fire, range: 800.0, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::MineLauncher }
    }

}

pub enum TurretClass {
    AutoCannon,
    BlastLaser,
    RocketLauncher,
    MineLauncher,
}