use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{util::Colour, resource::Fonts, component::*};

use super::AI;

pub fn spawn_drone(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "c",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 18.0,
                            color: Colour::ENEMY,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(5.0),
                engine: Engine::new(10.0, 10.0),
                health: Health::new(1, 4),
                collider: Collider { radius: 10.0 },
                ..Default::default()
            },
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(-PI),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 10 },
        ))
        .with_children(|parent| {
            // Custom short range blast laser
            parent.spawn(TurretBundle {
                class: TurretClass::BlastLaser,
                range: Range { max: 100.0 },
                fire_rate: FireRate::from_rate_in_seconds(2.0),
                damage: DoesDamage { amount: 1 },
                ..Default::default()
            });
        });
}