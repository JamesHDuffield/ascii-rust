use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{util::Colour, resource::Fonts, component::*};

use super::AI;

pub fn spawn_fighter(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: GlyphBundle::new("w", Colour::ENEMY, 18.0, fonts.primary.clone()),
                transform: Transform::from_translation(position),
                physics: Physics::new(5.0),
                engine: Engine::new(14.0, 14.0),
                health: Health::new(10, 0),
                collider: Collider { radius: 10.0 },
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 20.0,
                    size_max: 25.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 10 },
        ))
        .with_children(|parent| {
            parent.spawn(TurretBundle {
                class: TurretClass::AutoCannon,
                fire_rate: FireRate::from_rate_in_seconds(1.0),
                damage: DoesDamage::from_amount(2),
                ..Default::default()
            });
        });
}