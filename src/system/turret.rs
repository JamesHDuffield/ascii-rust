use crate::{colour, component::*};
use bevy::prelude::*;

fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity: Entity,
    position: Vec3,
    direction: Vec2,
) {
    let bullet_speed = 1000.0;
    commands.spawn((
        Bullet::new(3.2),
        Text2dBundle {
            text: Text::from_section(
                ".",
                TextStyle {
                    font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                    font_size: 12.0,
                    color: colour::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: position,
                ..Default::default()
            },
            ..default()
        },
        Physics {
            acceleration: Vec2::ZERO,
            velocity: direction * bullet_speed,
            drag: 0.0,
        },
        Collider { radius: 5.0 },
        Owner(entity),
    ));
}

pub fn turret_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(&mut Turret, &Parent), With<Turret>>,
    target_query: Query<(Entity, &Transform), (With<Targettable>, With<Transform>)>,
    parent_query: Query<(&Transform, Entity)>,
) {
    let potential_targets: Vec<(Entity, &Transform)> = target_query.iter().collect();
    for (mut turret, parent) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity)) = parent_query.get(parent.get())
        {
            if turret.target == None {
                // Look for a target
                let mut potentials_without_parent: Vec<&(Entity, &Transform)> = potential_targets
                    .iter()
                    .filter(|a| a.0 != parent_entity)
                    .collect();
                potentials_without_parent.sort_by(|a, b| {
                    a.1.translation
                        .distance(parent_transform.translation)
                        .partial_cmp(&b.1.translation.distance(parent_transform.translation))
                        .unwrap()
                });
                turret.target = potentials_without_parent
                    .first()
                    .map(|potential| potential.0);
                // Switching targets resets turret timer
                turret.timer.reset();
            }

            if let Some(target) = turret.target {
                turret.timer.tick(time.delta());
                // Shoot the target
                if turret.timer.just_finished() {
                    // Shoot a projectile towards the target
                    let target_translation =
                        target_query.get(target).map(|t| t.1.translation).unwrap();
                    let direction = (target_translation - parent_transform.translation).normalize();
                    spawn_bullet(
                        &mut commands,
                        &asset_server,
                        parent_entity,
                        parent_transform.translation.clone(),
                        direction.truncate(),
                    );
                }
            } else {
                turret.timer.reset();
            }
        }
    }
}
