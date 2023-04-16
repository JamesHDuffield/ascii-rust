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
    player_query: Query<(&Engine, &Health), (With<IsPlayer>, With<Engine>, With<Health>)>,
    mut query: Query<&Children, With<UINode>>,
    mut q_child: Query<&mut Text>,
) {
    if let Ok((engine, health)) = player_query.get_single() {
        let displays = vec![
            format!("{} {}", health.health, bar(health.health, health.max_health * 2, 10)),
            format!("{} {}", health.shield, bar(health.shield, health.max_shield, 10)),
            format!("{} m/s", engine.speed.round()),
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
