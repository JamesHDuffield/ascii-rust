use bevy::prelude::*;

use crate::{component::*, util::*, resource::Fonts};

use super::TurretFireEvent;

pub fn fire_mine_launcher(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &DoesDamage, &EffectSize, &EffectColour)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.iter() {
        match ev.class {
            TurretClass::MineLauncher => {

                // Get Turret Info
                let Ok((parent, damage, size, colour)) = turret_query.get(ev.turret) else { continue; };

                // Get Parent Info
                let Ok(parent_transform) = parent_query.get(parent.get()) else { continue; };

                // Spawn mine
                let origin = parent_transform.translation.truncate();
                commands.spawn((
                    Bullet::new(30.0),
                    Text2dBundle {
                        text: Text::from_section(
                            "+",
                            TextStyle {
                                font: fonts.primary.clone(),
                                font_size: 12.0,
                                color: colour.0,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform {
                            translation: origin.extend(RenderLayer::Bullet.as_z()),
                            ..Default::default()
                        },
                        ..default()
                    },
                    Health::new(1, 0),
                    Collider { radius: size.0 },
                    Owner(parent.get()),
                    ExplodesOnDespawn {
                        size_min: size.0,
                        size_max: size.0,
                        colour: colour.0,
                        ..Default::default()
                    },
                    AoeDamage { damage: damage.amount, range: size.0 },
                    DespawnWithScene,
                ));

            },
            _ => (),
        }
    }
}