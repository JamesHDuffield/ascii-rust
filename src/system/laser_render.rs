use bevy::prelude::*;

use crate::component::*;

pub fn laser_render_system(
  mut query: Query<(&Bullet, &mut Sprite), (With<LaserRender>, With<Bullet>, With<Sprite>)>,
) {
  for (bullet, mut sprite) in &mut query {
    const WIDTH: f32 = 2.0;
    let y = sprite.custom_size.map_or(0.0, |size| size.y);
    sprite.custom_size = Some(Vec2 { x: bullet.ttl.percent_left() * WIDTH, y }); 
  }
}