use bevy::prelude::*;

use crate::{component::*, util::Colour};



pub fn hit_flash_system(
  time: Res<Time>,
  mut query: Query<(&mut TextColor, &mut HitFlash)>,
) {
  for (mut text, mut hit_flash) in &mut query {
    // First time
    if !hit_flash.timer.paused() && hit_flash.timer.elapsed().is_zero() {
        // Store the actual colour once
        if hit_flash.original_colour.is_none() {
            hit_flash.original_colour = Some(text.0);
        }
        // Set to flash colour
        text.0 = hit_flash.flash_colour;
    }

    hit_flash.timer.tick(time.delta());
    
    // End
    if hit_flash.timer.just_finished() {
        // Reset to original colour
        text.0 = hit_flash.original_colour.unwrap_or(Colour::PURPLE);
        // Stop the timer
        hit_flash.timer.pause();
    }
  }
}