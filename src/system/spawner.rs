use std::{f32::consts::PI, cmp::min};

use crate::{colour, component::*, resource::{Fonts, Spawning, GameTime}, math::random_2d_unit_vector};
use bevy::prelude::*;

fn spawn_enemy(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    "w",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 32.0,
                        color: colour::ENEMY,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform {
                    translation: position,
                    scale: Vec3 {
                        x: 0.5,
                        y: 0.5,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            Physics::new(5.0),
            Engine::new(18.0, 18.0),
            Health::new(10, 0),
            Collider { radius: 5.0 },
            Targettable(Allegiance::ENEMY),
            WillTarget(vec![Allegiance::PLAYER]),
            AI,
            DropsLoot,
            ExplodesOnDespawn::default(),
            DespawnWithScene,
            WorthPoints { value: 10 },
        ))
        .with_children(|parent| {
            parent.spawn(TurretBundle::auto_cannon());
        });
}

pub fn spawner_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut spawning: ResMut<Spawning>,
    enemies_query: Query<Entity, With<AI>>,
    player_query: Query<&Transform, With<IsPlayer>>,
) {

    let seconds_since_start: u32 = (time.elapsed() - game_time.start_time).as_secs().try_into().unwrap_or_default();
    let difficulty = seconds_since_start / 30 + 1; // Goes from 1-20 difficulty in 10 minutes

    spawning.timer.tick(time.delta() * difficulty); // Spawns quicker as time goes on

    if spawning.timer.just_finished() {

        if let Ok(player_transformation) = player_query.get_single() {
            // pick a random location off screen from player
            const DISTANCE_OFFSCREEN: f32 = 1000.0;
            let spawn_point = player_transformation.translation.truncate() + random_2d_unit_vector() * DISTANCE_OFFSCREEN;

            // Get current total amount of enemies
            let num_enemies: u32 = enemies_query.iter().len().try_into().unwrap_or(spawning.max);

            let max_num_enemies_to_spawn = min(difficulty * 5, spawning.max - num_enemies); // Spawns more as time goes on

            for _ in 0..max_num_enemies_to_spawn {
                // Ensure they spawn in a pack not on top of eachother
                let jiggled_spawn = spawn_point + random_2d_unit_vector() * 10.0;
                spawn_enemy(&mut commands, &fonts, jiggled_spawn.extend(0.0));
            }
        }

    }
}
