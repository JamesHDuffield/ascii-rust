use crate::{colour, component::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Health, Entity, &Transform), With<Health>>,
) {
    for (mut health, entity, transform) in &mut query {
        if health.health <= 0 {
            death(&mut commands, entity, transform);
            continue;
        }
        // Recharge shield
        health.shield_recharge_cooldown.tick(time.delta());
        if health.shield_recharge_cooldown.finished() {
            health.shield_recharge_timer.tick(time.delta());
            if health.shield_recharge_timer.just_finished() {
                if health.shield == health.max_shield {
                    return;
                }
                health.shield += 1;
            }
        }
    }
}

fn death(commands: &mut Commands, entity: Entity, transform: &Transform) {
    // Despawn
    commands.entity(entity).despawn_recursive();
    // Spawn several explosions
    let mut rng = rand::thread_rng();
    for _ in 0..3 {
      let offset = Vec2 { x: rng.gen_range(-10.0..=10.0), y: rng.gen_range(-10.0..=10.0) };
      commands.spawn((
          ExplosionRender {
              origin: transform.translation.truncate() + offset,
              radius: rng.gen_range(20.0..=50.0),
              ttl: Timer::from_seconds(rng.gen_range(0.3..=0.4), TimerMode::Once),
          },
          ShapeBundle {
              path: GeometryBuilder::build_as(&shapes::Circle {
                  center: transform.translation.truncate(),
                  radius: 0.0,
              }),
              ..default()
          },
          Stroke::new(colour::RED, 1.0),
      ));
    }
}
