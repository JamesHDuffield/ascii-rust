use bevy::prelude::*;

use crate::{component::*, util::Colour};



pub fn hit_flash_system(
  time: Res<Time>,
  mut query: Query<(&mut Text, &mut Health)>,
) {
  for (mut text, mut health) in &mut query {
    // First time
    if !health.hit_flash_timer.paused() && health.hit_flash_timer.elapsed().is_zero() {
        // Store the actual colour once
        if health.original_colour.is_none() {
            health.original_colour = text.sections.first().and_then(|section| Some(section.style.color));
        }
        // Set to white
        text.sections.iter_mut().for_each(|section| { section.style.color = Colour::RED });
    }

    health.hit_flash_timer.tick(time.delta());
    
    // End
    if health.hit_flash_timer.just_finished() {
        // Reset to actual colour
        text.sections.iter_mut().for_each(|section| { section.style.color = health.original_colour.unwrap_or(Colour::PURPLE) });
        // Stop the timer
        health.hit_flash_timer.pause();
    }
  }
}