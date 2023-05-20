use std::f32::consts::PI;

use crate::{colour, component::*, resource::Fonts, layer::RenderLayer};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub fn turret_targetting_system(
    mut query: Query<(&mut Targets, &Parent), With<Targets>>,
    target_query: Query<(Entity, &Transform, &Targettable), (With<Targettable>, With<Transform>)>,
    parent_query: Query<(&Transform, Entity, &WillTarget), (With<Transform>, With<WillTarget>)>,
) {
    let potential_targets: Vec<(Entity, &Transform, &Targettable)> = target_query.iter().collect();
    for (mut targets, parent) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity, parent_will_target)) = parent_query.get(parent.get()) {
            if targets.target == None {
                // Look for a target
                let mut potentials_without_parent: Vec<&(Entity, &Transform, &Targettable)> = potential_targets
                    .iter()
                    .filter(|a| a.0 != parent_entity && parent_will_target.0.contains(&a.2.0))
                    .collect();
                potentials_without_parent.sort_by(|a, b| {
                    a.1.translation.truncate()
                        .distance(parent_transform.translation.truncate())
                        .partial_cmp(&b.1.translation.truncate().distance(parent_transform.translation.truncate()))
                        .unwrap()
                });
                targets.target = potentials_without_parent
                    .first()
                    .map(|potential| potential.0);
            }
        }
    }
}

pub fn turret_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    time: Res<Time>,
    mut query: Query<(&mut FireRate, &TurretClass, &mut Targets, &Parent), (With<TurretClass>, With<FireRate>, With<Targets>)>,
    parent_query: Query<(&Transform, Entity), With<Transform>>,
    mut existing_query: Query<(&Transform, Option<&mut Health>), With<Transform>>,
) {
    for (mut fire_rate, class, mut targets, parent) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity)) = parent_query.get(parent.get()) {
            if let Some(target) = targets.target {
                // Check target still exists and if not clear it
                match commands.get_entity(target) {
                    None => {
                        targets.target = None;
                        break;
                    }
                    Some(_) => (),
                }
                fire_rate.timer.tick(time.delta());
                if fire_rate.timer.just_finished() {
                    // Fire!
                    if let Ok((target_transform, target_health)) = existing_query.get_mut(target) {
                        let origin = parent_transform.translation.truncate();
                        match class {
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
                            TurretClass::MineLauncher => spawn_mine(
                                &mut commands,
                                &fonts,
                                parent_entity,
                                origin,
                            ),
                            TurretClass::ShrapnelCannon => spawn_shrapnel(
                                &mut commands,
                                &fonts,
                                parent_entity,
                                origin,
                                target_transform.translation.truncate(),
                            )
                        }
                    }
                }
            } else {
                fire_rate.timer.reset();
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
                translation: origin.extend(RenderLayer::Bullet.as_z()),
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
        DirectDamage(2),
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
            transform: Transform::from_xyz(0., 0., RenderLayer::Bullet.as_z()),
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
                translation: origin.extend(RenderLayer::Bullet.as_z()),
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
        AoeDamage { damage: 5, range: 40.0 },
        DespawnWithScene,
    ));
}

fn spawn_mine(
    commands: &mut Commands,
    fonts: &Res<Fonts>,
    owner: Entity,
    origin: Vec2,
) {
    commands.spawn((
        Bullet::new(30.0),
        Text2dBundle {
            text: Text::from_section(
                "+",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::WHITE,
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
        Collider { radius: 40.0 },
        Owner(owner),
        ExplodesOnDespawn::default(),
        AoeDamage { damage: 5, range: 40.0 },
        DespawnWithScene,
    ));
}


fn spawn_shrapnel(
    commands: &mut Commands,
    fonts: &Res<Fonts>,
    entity: Entity,
    origin: Vec2,
    target: Vec2,
) {
    const NUM_BULLETS: u8 = 16;
    const SPREAD: f32 = PI / 4.0;

    let mut rng = rand::thread_rng();
    let direction = (target - origin).normalize();
    for _ in 0..NUM_BULLETS {
        let random_angle = rng.gen_range(-SPREAD / 2.0..SPREAD / 2.0);
        let spread_direction = Vec2::from_angle(random_angle).rotate(direction);
        spawn_bullet(commands, fonts, entity, origin, origin + spread_direction);
    }
}