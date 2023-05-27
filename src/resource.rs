use bevy::prelude::*;
use std::{fmt, time::Duration};

#[derive(Resource)]
pub struct Fonts {
    pub primary: Handle<Font>,
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

#[derive(Resource)]
pub struct GameTime {
    pub start_time: Duration,
}

#[derive(Resource)]
pub struct PlayerLevel {
    pub value: u32,
}

impl PlayerLevel {
    pub fn required_cargo_to_level(&self) -> u32 {
        self.value * 1 // TODO make exponential?
    }
}