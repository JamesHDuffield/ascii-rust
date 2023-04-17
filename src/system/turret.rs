use bevy::prelude::*;
use crate::{component::*, colour};

pub fn turret_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  time: Res<Time>,
  mut query: Query<(&mut Turret, &Parent), With<Turret>>,
  q_parent: Query<(&Transform, Option<&BaseGlyphRotation>)>,
) {
  for (mut turret, parent) in &mut query {
    turret.timer.tick(time.delta());
    if turret.timer.just_finished() {
      // Get parent (ship)
      if let Ok((parent_transform, base_rotation)) = q_parent.get(parent.get()) {
        // Shoot a projectile straight out in front
        let mut direction = parent_transform.rotation.mul_vec3(Vec3::NEG_X);
        if let Some(base_rotation) = base_rotation {
          direction = base_rotation.rotation.mul_vec3(direction);
        }
        direction *= 1000.0; // Bullet speed
        commands.spawn((
          Bullet::new(3.2),
          Text2dBundle {
            text: Text::from_section(".", TextStyle {
              font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
              font_size: 12.0,
              color: colour::WHITE,
          }).with_alignment(TextAlignment::Center),
            transform: Transform { translation: parent_transform.translation.clone(), ..Default::default() },
            ..default()
          },
          Physics { acceleration: Vec2::ZERO, velocity: direction.truncate(), drag: 0.0 },
          Collider { radius: 5.0, layer: 0b00000000, mask: 0b00000010 },
        ));
      }
    }
  }
}