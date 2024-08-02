use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::component::*;

pub fn explosion_render_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut ExplosionRender, &mut Path, Entity, &mut Stroke), Without<ShouldDespawn>>,
) {
  for (mut explosion, mut path, entity, mut stroke) in &mut query {
    explosion.ttl.tick(time.delta());
    
    let shape = shapes::Circle {
        center: explosion.origin,
        radius: explosion.radius * explosion.ttl.percent(),
    };
    *path = GeometryBuilder::build_as(&shape);

    if explosion.fade_out {
      stroke.color.set_a(explosion.ttl.percent_left()); 
    }

    if explosion.ttl.finished() {
      commands.entity(entity).insert(ShouldDespawn);
    }

  }
}