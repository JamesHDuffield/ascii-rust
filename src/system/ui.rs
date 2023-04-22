use bevy::prelude::*;
use crate::component::*;

fn bar(current: i32, max: i32, width: i32) -> String {
  let bars: usize = match (current.clamp(0, max) * width / max).try_into() {
    Ok(val) => val,
    Err(_) => 0,
  };
  format!("{}{}", String::from('|').repeat(bars), String::from('.').repeat(width as usize - bars))
}

pub fn ui_system(
    player_query: Query<(&Engine, &Health, &Cargo), (With<IsPlayer>, With<Engine>, With<Health>, With<Cargo>)>,
    mut query: Query<&Children, With<UINode>>,
    mut q_child: Query<&mut Text>,
) {
    if let Ok((engine, health, cargo)) = player_query.get_single() {
        let displays = vec![
            format!("Armor  {} {}", bar(health.health, health.max_health, 10), health.health),
            format!("Shield {} {}", bar(health.shield, health.max_shield, 10), health.shield),
            format!("Speed  {} m/s", engine.speed.round()),
            format!("Cargo  {} scrap", cargo.0),
        ];

        // Loop over children and update display values
        for children in &mut query {
            for (i, display) in displays.iter().enumerate() {
                if let Some(&child) = children.get(i) {
                    if let Ok(mut text) = q_child.get_mut(child) {
                        if let Some(mut section) = text.sections.get_mut(0) {
                            section.value = display.to_string();
                        }
                    }
                }
            }
        }
    }
}
