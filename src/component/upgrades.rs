use bevy::prelude::Component;

// Used to randomly select
pub enum UpgradeOption {
    Speed,
    Magnet,
    ShieldRecharge,
    ShieldCooldown,
}

#[derive(Component)]
pub struct Upgrades {
    // pub fire_rate: u8,
    // pub damage: u8,
    pub speed: u8,
    pub magnet: u8,
    // pub damage_reduction: u8,
    // pub max_shield: u8,
    pub shield_recharge: u8,
    pub shield_cooldown: u8,
    // pub aoe_size: u8,
    // pub projectile_speed: u8,
    // pub projectile_quantity: u8,
}

impl Upgrades {
    pub fn new() -> Upgrades {
        Upgrades { speed: 0, magnet: 0, shield_recharge: 0, shield_cooldown: 0 }
    }

    pub fn display_for_ui(&self) -> Vec<String> {
        vec![
            ("Speed", self.speed),
            ("Magnet", self.magnet),
            ("Shield Boost", self.shield_recharge),
            ("Quick Shield", self.shield_cooldown),
        ]
        .iter()
        .filter(|up| up.1 > 0)
        .map(|up| format!("{:0>2} {:>16}", up.1, up.0))
        .collect()
    }
}