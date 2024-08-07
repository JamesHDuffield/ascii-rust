use bevy::{prelude::*, time::Stopwatch};
use std::fmt;

use crate::component::Damage;

#[allow(dead_code)]
#[derive(Resource)]
pub struct Fonts {
    pub primary: Handle<Font>,
    pub unicode: Handle<Font>,
}

#[derive(Resource)]
pub struct Points {
    pub value: u32,
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Resource, Default)]
pub struct GameTime(pub Stopwatch);

#[derive(Resource)]
pub struct PlayerLevel {
    pub value: u32,
}

impl PlayerLevel {
    pub fn required_cargo_to_level(&self) -> u32 {
        self.value * 4 // TODO make exponential?
    }
}

#[derive(Event)]
pub struct TakeDamageEvent { 
    pub entity: Entity,
    pub damage: Damage,
}