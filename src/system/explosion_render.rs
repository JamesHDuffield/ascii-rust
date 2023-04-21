use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::component::*;

pub fn explosion_render_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut ExplosionRender, &mut Path, Entity), (With<ExplosionRender>, With<Path>)>,
) {
  for (mut explosion, mut path, entity) in &mut query {
    explosion.ttl.tick(time.delta());
    let shape = shapes::Circle {
        center: explosion.origin,
        radius: explosion.radius * explosion.ttl.percent(),
    };
    *path = GeometryBuilder::build_as(&shape);
    if explosion.ttl.finished() {
      commands.entity(entity).despawn();
    }
  }
}