use std::f32::consts::PI;

use crate::{colour, component::*, resource::Fonts};
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
        ))
        .with_children(|parent| {
            parent.spawn(Turret::auto_cannon());
        });
}

pub fn spawner_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    time: Res<Time>,
    mut query: Query<(&mut Spawner, &Transform), With<Spawner>>,
) {
    for (mut spawner, transform) in &mut query {
        spawner.cooldown_timer.tick(time.delta());
        if spawner.cooldown_timer.just_finished() {
            spawn_enemy(&mut commands, &fonts, transform.translation);
        }
    }
}
