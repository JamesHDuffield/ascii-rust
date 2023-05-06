use std::f32::consts::PI;

use crate::{colour, component::*, resource::Fonts};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn turret_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    time: Res<Time>,
    mut query: Query<(&mut Turret, &Parent), With<Turret>>,
    target_query: Query<(Entity, &Transform, &Targettable), (With<Targettable>, With<Transform>)>,
    parent_query: Query<(&Transform, Entity, &WillTarget), (With<Transform>, With<WillTarget>)>,
    mut existing_query: Query<(&Transform, Option<&mut Health>), With<Transform>>,
) {
    let potential_targets: Vec<(Entity, &Transform, &Targettable)> = target_query.iter().collect();
    for (mut turret, parent) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity, parent_will_target)) = parent_query.get(parent.get()) {
            if turret.target == None {
                // Look for a target
                let mut potentials_without_parent: Vec<&(Entity, &Transform, &Targettable)> = potential_targets
                    .iter()
                    .filter(|a| a.0 != parent_entity && parent_will_target.0.contains(&a.2.0))
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
                    }
                    Some(_) => (),
                }
                turret.timer.tick(time.delta());
                if turret.timer.just_finished() {
                    // Fire!
                    if let Ok((target_transform, target_health)) = existing_query.get_mut(target) {
                        let origin = parent_transform.translation.truncate();
                        match turret.class {
                            TurretClass::AutoCannon => spawn_bullet(
                                &mut commands,
                                &fonts,
                                parent_entity,
                                origin,
                                target_transform.translation.truncate(),
                            ),
                            TurretClass::BlastLaser => spawn_laser(
                                &mut commands,
                                parent_entity,
                                origin,
                                target_transform.translation.truncate(),
                                target_health,
                            ),
                            TurretClass::RocketLauncher => spawn_rocket(
                                &mut commands,
                                &fonts,
                                parent_entity,
                                origin,
                                target,
                            ),
                        }
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
    fonts: &Res<Fonts>,
    entity: Entity,
    origin: Vec2,
    target: Vec2,
) {
    let bullet_speed = 1000.0;
    let direction = (target - origin).normalize();
    commands.spawn((
        Bullet::new(1.2),
        Text2dBundle {
            text: Text::from_section(
                ".",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::RED,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: origin.extend(0.0),
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
        DespawnWithScene,
    ));
}

fn spawn_laser(
    commands: &mut Commands,
    entity: Entity,
    origin: Vec2,
    target: Vec2,
    target_health: Option<Mut<Health>>,
) {
    commands.spawn((
        Bullet::new(0.1),
        LaserRender,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(origin, target)),
            ..default()
        },
        Stroke::new(colour::RED, 1.0),
        Owner(entity),
        DespawnWithScene,
    ));
    // Immediate hit
    if let Some(mut health) = target_health {
        health.take_damage(1);
    }
}

fn spawn_rocket(
    commands: &mut Commands,
    fonts: &Res<Fonts>,
    owner: Entity,
    origin: Vec2,
    target: Entity,
) {
    commands.spawn((
        Bullet::new(3.0),
        Text2dBundle {
            text: Text::from_section(
                "!",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: origin.extend(0.0),
                ..Default::default()
            },
            ..default()
        },
        BaseGlyphRotation {
            rotation: Quat::from_rotation_z(PI / 2.0),
        },
        Physics {
            acceleration: Vec2::ZERO,
            velocity: Vec2::ZERO,
            drag: 0.0,
        },
        Engine::new_with_steering(40.0, 10.0, 0.5),
        Seeker(target),
        Collider { radius: 5.0 },
        Owner(owner),
        ExplodesOnDespawn::default(),
        DespawnWithScene,
    ));
}
