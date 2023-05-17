use crate::{colour, component::*, resource::{Fonts, Points}, GameState};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
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
            Option<&ExplodesOnDespawn>,
            Option<&WorthPoints>,
        ),
        With<ShouldDespawn>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut points: ResMut<Points>,
) {
    for (entity, drops_loot, transform, is_player, explodes, worth_points) in &mut query {
        commands.entity(entity).despawn_recursive();

        if let Some(transform) = transform {
            if let Some(_drops_loot) = drops_loot {
                spawn_loot(&mut commands, &fonts, transform.translation);
            }
            if let Some(explodes) = explodes {
                explode(&mut commands, explodes, transform.translation.truncate());
            }
        }

        if let Some(worth_points) = worth_points {
            points.value += worth_points.value;
        }

        if let Some(_) = is_player {
            game_state.set(GameState::GameOver);
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
                DespawnWithScene,
                WorthPoints { value: 1 },
            )
        })
        .collect::<Vec<_>>();
    commands.spawn_batch(loots);
}

fn explode(commands: &mut Commands, explodes: &ExplodesOnDespawn, position: Vec2) {
    // Spawn several explosions
    let mut rng = rand::thread_rng();
    for _ in explodes.amount_min..=explodes.amount_max {
      let offset = Vec2 { x: rng.gen_range(-explodes.spread..=explodes.spread), y: rng.gen_range(-explodes.spread..=explodes.spread) };
      commands.spawn((
          ExplosionRender {
              origin: position + offset,
              radius: rng.gen_range(explodes.size_min..=explodes.size_max),
              ttl: Timer::from_seconds(rng.gen_range(explodes.duration_min..=explodes.duration_max), TimerMode::Once),
          },
          ShapeBundle {
              path: GeometryBuilder::build_as(&shapes::Circle {
                  center: position,
                  radius: 0.0,
              }),
              ..default()
          },
          Stroke::new(explodes.colour, 1.0),
      ));
    }
}