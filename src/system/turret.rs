use bevy::prelude::*;
use crate::{component::*, colour};

fn spawn_bullet(commands: &mut Commands, asset_server: &Res<AssetServer>, entity: Entity, position: Vec3, direction: Vec2) {
  let bullet_speed = 1000.0;
  commands.spawn((
    Bullet::new(3.2),
    Text2dBundle {
      text: Text::from_section(".", TextStyle {
        font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
        font_size: 12.0,
        color: colour::WHITE,
    }).with_alignment(TextAlignment::Center),
      transform: Transform { translation: position, ..Default::default() },
      ..default()
    },
    Physics { acceleration: Vec2::ZERO, velocity: direction * bullet_speed, drag: 0.0 },
    Collider { radius: 5.0 },
    Owner(entity),
  ));
}

pub fn turret_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  time: Res<Time>,
  mut query: Query<(&mut Turret, &Parent), With<Turret>>,
  q_parent: Query<(&Transform, Option<&BaseGlyphRotation>, Entity)>,
) {
  for (mut turret, parent) in &mut query {
    turret.timer.tick(time.delta());
    if turret.timer.just_finished() {
      // Get parent (ship)
      if let Ok((parent_transform, base_rotation, parent_entity)) = q_parent.get(parent.get()) {
        // Shoot a projectile straight out in front
        let mut direction = parent_transform.rotation.mul_vec3(Vec3::NEG_X);
        if let Some(base_rotation) = base_rotation {
          direction = base_rotation.rotation.mul_vec3(direction);
        }
        spawn_bullet(&mut commands, &asset_server, parent_entity, parent_transform.translation.clone(), direction.truncate());
      }
    }
  }
}