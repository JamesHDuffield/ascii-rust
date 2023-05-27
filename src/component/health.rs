use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: i32,
    pub shield: i32,
    pub max_health: i32,
    pub max_shield: i32,
    pub shield_recharge_cooldown: Timer,
    pub shield_recharge_timer: Timer,

    // Hit Flash
    pub hit_flash_timer: Timer,
    pub original_colour: Option<Color>,
}

impl Default for Health {
    fn default() -> Self { Health::new(100, 100) }
}

impl Health {
    pub fn new(max_health: i32, max_shield: i32) -> Health {
        let mut hit_flash_timer = Timer::from_seconds(0.1, TimerMode::Once);
        hit_flash_timer.pause();
        Health {
            health: max_health,
            max_health,
            shield: max_shield,
            max_shield,
            shield_recharge_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            shield_recharge_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            hit_flash_timer,
            original_colour: None,
        }
    }

    fn hit_flash(&mut self) {
        self.hit_flash_timer.reset();
        self.hit_flash_timer.unpause();
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
        Self::hit_flash(self);
    }
}
