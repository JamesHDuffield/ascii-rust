use bevy::prelude::Component;

#[derive(Component)]
pub struct Upgrades {
    // pub fire_rate: u8,
    // pub damage: u8,
    pub speed: u8,
    // pub max_armor: u8,
    pub magnet: u8,
    // pub damage_reduction: u8,
    // pub max_shield: u8,
    // pub shield_recharge: u8,
    // pub shield_cooldown: u8,
    // pub aoe_size: u8,
    // pub projectile_speed: u8,
    // pub projectile_quantity: u8,
}

impl Upgrades {
    pub fn new() -> Upgrades {
        Upgrades { speed: 0, magnet: 0 }
    }
}