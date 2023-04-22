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

    pub fn blast_laser() -> Turret {
        let rate_of_fire = 5.0;
        Turret { target: None, rate_of_fire, range: 200.0, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::BlastLaser }
    }

    pub fn new(rate_of_fire: f32, range: f32) -> Turret {
        Turret { target: None, rate_of_fire, range, timer: Timer::from_seconds(1.0 / rate_of_fire, TimerMode::Repeating), class: TurretClass::AutoCannon }
    }

}

pub enum TurretClass {
    AutoCannon,
    BlastLaser,
}