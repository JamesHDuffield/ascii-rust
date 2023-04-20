use std::f32::consts::PI;

use crate::{colour, component::*, math};
use bevy::prelude::*;

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
                // Check target still exists and if not clear it
                match commands.get_entity(target) {
                    None => {
                        turret.target = None;
                        break;
                    },
                    Some(_) => ()
                }
                turret.timer.tick(time.delta());
                if turret.timer.just_finished() {
                    // Fire!
                    let target_translation =
                        target_query.get(target).map(|t| t.1.translation).unwrap_or(Vec3::X);
                    let direction = (target_translation - parent_transform.translation).normalize();
                    match turret.class {
                        TurretClass::AutoCannon => spawn_bullet(&mut commands, &asset_server, parent_entity, parent_transform.translation.clone(), direction.truncate()),
                        TurretClass::BlastLaser => spawn_laser(&mut commands, parent_entity, parent_transform.translation.truncate(), target_translation.truncate()),
                    }

                }
            } else {
                turret.timer.reset();
            }
        }
    }
}

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

fn spawn_laser(
    commands: &mut Commands,
    entity: Entity,
    origin: Vec2,
    target: Vec2,
) {
    let distance = target.distance(origin);
    let direction = (target - origin).normalize();
    commands.spawn((
        Bullet::new(0.1),
        LaserRender,
        SpriteBundle {
            sprite: Sprite {
                color: colour::RED,
                anchor: bevy::sprite::Anchor::BottomCenter,
                custom_size: Some(Vec2::new(2.0, distance)),
                ..default()
            },
            transform: Transform { translation: origin.extend(0.0), rotation: math::quaternion_from_2d_vector(direction) * Quat::from_rotation_z(-PI / 2.0), ..Default::default()},
            ..default()
        },
        Owner(entity),
    ));
    // Immediate hit TODO
}
