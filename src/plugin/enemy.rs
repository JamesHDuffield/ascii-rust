mod fighter;
mod drone;
mod drone_boss;
mod mothership;

use self::fighter::*;
use self::drone::*;
use self::drone_boss::*;
use self::mothership::*;

use std::{cmp::min, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::{component::*, resource::*, util::*, AppState, GameState, game_not_paused};

#[derive(Resource)]
pub struct Spawning {
    pub max: u32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct AI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_startup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                ai_system
                    .run_if(game_not_paused)
                    .in_set(OnUpdate(AppState::InGame)),
            )
            // Stop when game over
            .add_system(
                spawner_system
                    .run_if(in_state(GameState::Running))
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}

fn spawn_startup(mut commands: Commands) {
    // Set spawn limit
    let seconds = 30.0;
    let mut timer = Timer::from_seconds(seconds, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs_f32(seconds));
    commands.insert_resource(Spawning { max: 100, timer });
}

fn spawner_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut spawning: ResMut<Spawning>,
    enemies_query: Query<Entity, With<AI>>,
    player_query: Query<&Transform, With<IsPlayer>>,
) {
    let seconds_since_start: u32 = (time.elapsed() - game_time.start_time)
        .as_secs()
        .try_into()
        .unwrap_or_default();
    let difficulty = seconds_since_start / 30 + 1; // Goes from 1-20 difficulty in 10 minutes

    spawning.timer.tick(time.delta() * difficulty); // Spawns quicker as time goes on

    if spawning.timer.just_finished() {
        if let Ok(player_transformation) = player_query.get_single() {
            // pick a random location off screen from player
            const DISTANCE_OFFSCREEN: f32 = 1000.0;
            let spawn_point = player_transformation.translation.truncate()
                + Math::random_2d_unit_vector() * DISTANCE_OFFSCREEN;

            // Get current total amount of enemies
            let num_enemies: u32 = enemies_query
                .iter()
                .len()
                .try_into()
                .unwrap_or(spawning.max);

            let max_num_enemies_to_spawn = min(difficulty * 5, spawning.max - num_enemies); // Spawns more as time goes on

            for _ in 0..max_num_enemies_to_spawn {
                // Ensure they spawn in a pack not on top of eachother
                let jiggled_spawn = spawn_point + Math::random_2d_unit_vector() * 10.0;
                let spawn_func = match rand::thread_rng().gen_range(0..100) {
                    0 => spawn_mothership,
                    1..=5 => spawn_drone_boss,
                    6..=15 => spawn_fighter,
                    _ => spawn_drone,
                };
                spawn_func(
                    &mut commands,
                    &fonts,
                    jiggled_spawn.extend(RenderLayer::Enemy.as_z()),
                );
                
            }
        }
    }
}

fn ai_system(
    mut query: Query<(&Transform, &mut Engine, Entity), (With<AI>, With<Transform>, With<Engine>)>,
    other_query: Query<(&Transform, &Physics, Entity), (With<AI>, With<Transform>, With<Physics>)>,
    player_query: Query<&Transform, (With<IsPlayer>, With<Transform>, Without<AI>)>,
) {
    const PROXIMITY_CUTOFF: f32 = 20.0;
    const LOOK_AHEAD: f32 = 10.0;
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut engine, entity) in &mut query {
            let neighbours: Vec<Vec2> = other_query
                .iter()
                .filter(|other| other.2 != entity)
                .filter(|other| {
                    other
                        .0
                        .translation
                        .truncate()
                        .distance(transform.translation.truncate())
                        < 50.0
                })
                .map(|other| other.0.translation.truncate())
                .collect();
            let to_target =
                player_transform.translation.truncate() - transform.translation.truncate();

            let target_direction = if to_target.length() < PROXIMITY_CUTOFF {
                Vec2::ZERO
            } else {
                to_target.normalize_or_zero()
            };

            let seperation_direction = seperation(transform.translation.truncate(), &neighbours);
            let direction = (target_direction + seperation_direction).normalize_or_zero();

            engine.method = EngineMethod::Approach;

            if direction.length() > 0.0 {
                engine.target = Some(transform.translation.truncate() + direction * LOOK_AHEAD);
            } else {
                engine.target = None;
            };
        }
    }
}

fn seperation(position: Vec2, neighbours: &Vec<Vec2>) -> Vec2 {
    if neighbours.is_empty() {
        return Vec2::ZERO;
    }
    let away: Vec2 = neighbours
        .iter()
        .map(|neighbour| position - *neighbour)
        .sum();
    away.normalize_or_zero()
}
