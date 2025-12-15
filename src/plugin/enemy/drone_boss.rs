use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{util::Colour, resource::Fonts, component::*};

use super::AI;

pub fn spawn_drone_boss(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: GlyphBundle::new("C", Colour::ENEMY, 32.0, fonts.primary.clone()),
                transform: Transform::from_translation(position),
                physics: Physics::new(8.0),
                engine: Engine::new(8.0, 8.0),
                health: Health::new(10, 40),
                collider: Collider { radius: 30.0 },
                ..Default::default()
            },
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(-PI),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 50 },
        ))
        .with_children(|parent| {
            // Custom short range blast laser
            parent.spawn(TurretBundle {
                class: TurretClass::BlastLaser,
                range: Range { max: 150.0 },
                fire_rate: FireRate::from_rate_in_seconds(1.0),
                damage: DoesDamage::from_amount(1),
                ..Default::default()
            });
        });
}