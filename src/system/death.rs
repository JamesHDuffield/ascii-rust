use crate::{colour, component::*};
use bevy::prelude::*;
use rand::prelude::*;

pub fn death_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, Option<&DropsLoot>, Option<&Transform>), With<ShouldDespawn>>,
) {
    for (entity, drops_loot, transform) in &mut query {
        commands.entity(entity).despawn_recursive();

        if let Some(transform) = transform {
            if let Some(drops_loot) = drops_loot {
                spawn_loot(&mut commands, &asset_server, transform.translation);
            }
        }
    }
}

fn spawn_loot(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    let mut rng = rand::thread_rng();
    let loots = (0..rng.gen_range(1..=3))
        .map(|_| {
            (
                IsLoot,
                Text2dBundle {
                    text: Text::from_section(
                        "*",
                        TextStyle {
                            font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                            font_size: 12.0,
                            color: colour::INACTIVE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Physics { acceleration: Vec2 { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0) }.normalize_or_zero() * rng.gen_range(50.0..100.0), drag: 1.0, velocity: Vec2::ZERO },
                Collider { radius: 20.0 },
            )
        })
        .collect::<Vec<_>>();
    commands.spawn_batch(loots);
}
