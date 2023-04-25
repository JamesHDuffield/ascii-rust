use crate::{colour, component::*, resource::Fonts};
use bevy::prelude::*;
use rand::prelude::*;

pub fn death_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut query: Query<
        (
            Entity,
            Option<&DropsLoot>,
            Option<&Transform>,
            Option<&IsPlayer>,
        ),
        With<ShouldDespawn>,
    >,
) {
    for (entity, drops_loot, transform, is_player) in &mut query {
        commands.entity(entity).despawn_recursive();

        if let Some(transform) = transform {
            if let Some(drops_loot) = drops_loot {
                spawn_loot(&mut commands, &fonts, transform.translation);
            }
        }

        if let Some(_) = is_player {
            game_over(&mut commands);
        }
    }
}

fn spawn_loot(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    let mut rng = rand::thread_rng();
    let loots = (0..rng.gen_range(1..=3))
        .map(|_| {
            (
                IsLoot,
                Text2dBundle {
                    text: Text::from_section(
                        "*",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 12.0,
                            color: colour::INACTIVE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Physics {
                    acceleration: Vec2 {
                        x: rng.gen_range(-1.0..1.0),
                        y: rng.gen_range(-1.0..1.0),
                    }
                    .normalize_or_zero()
                        * rng.gen_range(50.0..100.0),
                    drag: 1.0,
                    velocity: Vec2::ZERO,
                },
                Collider { radius: 20.0 },
            )
        })
        .collect::<Vec<_>>();
    commands.spawn_batch(loots);
}

fn game_over(commands: &mut Commands) {}