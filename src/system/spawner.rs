use std::f32::consts::PI;

use crate::{colour, component::*};
use bevy::prelude::*;

fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    "w",
                    TextStyle {
                        font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
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
            Engine::new(10.0, 10.0),
            Health::new(60, 20),
            Collider { radius: 5.0 },
            Targettable(Allegiance::ENEMY),
            WillTarget(vec![Allegiance::PLAYER]),
            AI,
            DropsLoot,
        ))
        .with_children(|parent| {
            parent.spawn(Turret::new(1.0, 200.0));
        });
}

pub fn spawner_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(&mut Spawner, &Transform), With<Spawner>>,
) {
    for (mut spawner, transform) in &mut query {
        spawner.cooldown_timer.tick(time.delta());
        if spawner.cooldown_timer.just_finished() {
            spawn_enemy(&mut commands, &asset_server, transform.translation);
        }
    }
}
