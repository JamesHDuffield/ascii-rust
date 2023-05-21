use bevy::prelude::*;
#[derive(Component)]
pub struct IsLoot;

#[derive(Component)]
pub struct DropsLoot;

#[derive(Component)]
pub struct Cargo(pub u32);

impl Cargo {
    pub fn default() -> Cargo {
        Cargo(0)
    }
}

#[derive(Component)]
pub struct Magnet {
  pub range: f32,
  pub strength: f32,
}

impl Magnet {
  pub fn default() -> Magnet {
    Magnet { range: 500.0, strength: 5.0 }
  }
}