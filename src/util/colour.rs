use bevy::{color::Srgba, prelude::Color};

pub struct Colour;

#[allow(dead_code)]
impl Colour {
    pub const BLACK: Color = Color::Srgba (Srgba { red: 31.0 / 255.0, green: 26.0 / 255.0, blue: 36.0 / 255.0, alpha: 1.0 });
    pub const PLAYER: Color = Color::Srgba (Srgba { red: 168.0 / 255.0, green: 207.0 / 255.0, blue: 218.0 / 255.0, alpha: 1.0 });
    pub const WHITE: Color = Color::Srgba (Srgba { red: 238.0 / 255.0, green: 236.0 / 255.0, blue: 222.0 / 255.0, alpha: 1.0 });
    pub const SHIELD: Color = Color::Srgba (Srgba { red: 120.0 / 255.0, green: 149.0 / 255.0, blue: 171.0 / 255.0, alpha: 1.0 });
    pub const INACTIVE: Color = Color::Srgba (Srgba { red: 119.0 / 255.0, green: 117.0 / 255.0, blue: 103.0 / 255.0, alpha: 1.0 });
    pub const ENEMY: Color = Color::Srgba (Srgba { red: 172.0 / 255.0, green: 138.0 / 255.0, blue: 113.0 / 255.0, alpha: 1.0 });
    pub const RED: Color = Color::Srgba (Srgba { red: 255.0 / 255.0, green: 138.0 / 255.0, blue: 113.0 / 255.0, alpha: 1.0 });
    pub const GREEN: Color = Color::Srgba (Srgba { red: 130.0 / 255.0, green: 170.0 / 255.0, blue: 119.0 / 255.0, alpha: 1.0 });
    pub const YELLOW: Color = Color::Srgba (Srgba { red: 237.0 / 255.0, green: 225.0 / 255.0, blue: 158.0 / 255.0, alpha: 1.0 });
    pub const PURPLE: Color = Color::Srgba (Srgba { red: 138.0 / 255.0, green: 112.0 / 255.0, blue: 225.0 / 255.0, alpha: 1.0 });
    pub const PINK: Color = Color::Srgba (Srgba { red: 255.0 / 255.0, green: 113.0 / 255.0, blue: 159.0 / 255.0, alpha: 1.0 });
}