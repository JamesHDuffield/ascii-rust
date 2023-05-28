use bevy::prelude::*;

use crate::{component::*, util::Colour};



pub fn hit_flash_system(
  time: Res<Time>,
  mut query: Query<(&mut Text, &mut HitFlash)>,
) {
  for (mut text, mut hit_flash) in &mut query {
    // First time
    if !hit_flash.timer.paused() && hit_flash.timer.elapsed().is_zero() {
        // Store the actual colour once
        if hit_flash.original_colour.is_none() {
            hit_flash.original_colour = text.sections.first().and_then(|section| Some(section.style.color));
        }
        // Set to flash colour
        text.sections.iter_mut().for_each(|section| { section.style.color = hit_flash.flash_colour });
    }

    hit_flash.timer.tick(time.delta());
    
    // End
    if hit_flash.timer.just_finished() {
        // Reset to original colour
        text.sections.iter_mut().for_each(|section| { section.style.color = hit_flash.original_colour.unwrap_or(Colour::PURPLE) });
        // Stop the timer
        hit_flash.timer.pause();
    }
  }
}