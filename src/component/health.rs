use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: i32,
    pub shield: i32,
    pub max_health: i32,
    pub max_shield: i32,
    pub shield_recharge_cooldown: Timer,
    pub shield_recharge_timer: Timer,
}

impl Default for Health {
    fn default() -> Self { Health::new(100, 100) }
}

impl Health {
    pub fn new(max_health: i32, max_shield: i32) -> Health {
        Health {
            health: max_health,
            max_health,
            shield: max_shield,
            max_shield,
            shield_recharge_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            shield_recharge_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.shield_recharge_cooldown.reset();
        self.shield_recharge_timer.reset();
        if amount > self.shield {
            self.health -= amount - self.shield;
            self.shield = 0;
        } else {
            self.shield -= amount;
        }
    }
}
