use bevy::prelude::*;
use bevy_prototype_lyon::prelude::Stroke;

use crate::component::*;

pub fn laser_render_system(
  mut query: Query<(&Bullet, &mut Stroke), (With<LaserRender>, With<Bullet>, With<Stroke>)>,
) {
  for (bullet, mut stroke) in &mut query {
    stroke.color.set_alpha(bullet.ttl.fraction_remaining()); 
  }
}