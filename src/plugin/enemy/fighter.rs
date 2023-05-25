use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{util::Colour, resource::Fonts, component::*};

use super::AI;

pub fn spawn_fighter(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "w",
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
                engine: Engine::new(18.0, 18.0),
                health: Health::new(10, 0),
                collider: Collider { radius: 10.0 },
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
            parent.spawn(TurretBundle::auto_cannon());
        });
}