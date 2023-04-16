use bevy::prelude::*;
use crate::component::*;

pub fn bullet_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut Bullet, Entity), With<Bullet>>,
) {
  for (mut bullet, entity) in &mut query {
    bullet.ttl.tick(time.delta());
    if bullet.ttl.just_finished() {
      commands.entity(entity).despawn();
    }
  }
}