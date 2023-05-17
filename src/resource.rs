use bevy::prelude::*;
use std::fmt;

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
pub struct Spawning {
    pub max: u32,
    pub batch_size: u32,
    pub timer: Timer,
}