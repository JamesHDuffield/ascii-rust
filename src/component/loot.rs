use bevy::prelude::*;
#[derive(Component)]
pub struct IsLoot;

#[derive(Component)]
pub struct DropsLoot;

#[derive(Component)]
pub struct Cargo(u32);

impl Cargo {
    pub fn new() -> Cargo {
        Cargo(0)
    }
}

#[derive(Component)]
pub struct Magnet {
  pub range: f32,
  pub strength: f32,
}