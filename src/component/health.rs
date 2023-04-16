use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: i32,
    pub shield: i32,
    pub max_health: i32,
    pub max_shield: i32
}

impl Health {

    pub fn new(max_health: i32, max_shield: i32) -> Health {
      Health { health: max_health, max_health, shield: max_shield, max_shield }
    }

    pub fn take_damage(&mut self, amount: i32) {
      if amount > self.shield {
        self.health -= amount - self.shield;
        self.shield = 0;
      } else {
        self.shield -= amount;
      }
    }

}