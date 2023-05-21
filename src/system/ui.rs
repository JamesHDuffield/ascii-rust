use bevy::prelude::*;
use crate::{component::*, resource::PlayerLevel, upgrade::PlayerUpgrades};

fn bar(current: i32, max: i32, width: i32) -> String {
  let bars: usize = match (current.clamp(0, max) * width / max).try_into() {
    Ok(val) => val,
    Err(_) => 0,
  };
  format!("{}{}", String::from('|').repeat(bars), String::from('.').repeat(width as usize - bars))
}

pub fn ui_system(
    upgrades: Res<PlayerUpgrades>,
    player_query: Query<(&Engine, &Health, &Cargo, &Children), With<IsPlayer>>,
    turret_query: Query<(&FireRate, &TurretClass)>,
    mut query: Query<(&Children, &UINode)>,
    mut q_child: Query<&mut Text>,
    level: Res<PlayerLevel>,
) {
    if let Ok((engine, health, cargo, turrets)) = player_query.get_single() {
        // Loop over children and update display values
        for (children, ui_node) in &mut query {

            let displays = match ui_node {
                UINode::Status => vec![
                    format!("{:<8} {} {}", "Armor", bar(health.health, health.max_health, 10), health.health),
                    format!("{:<8} {} {}", "Shield", bar(health.shield, health.max_shield, 10), health.shield),
                    format!("{:<8} {} {:0>2}", "Level", bar(cargo.0 as i32, level.required_cargo_to_level() as i32, 10), level.value),
                    format!("{:<8} {} m/s", "Speed", engine.speed.round()),
                ],
                UINode::Equipment => { 
                    let mut display = turrets
                        .iter()
                        .map(|e| turret_query.get(*e))
                        .filter_map(|result| result.ok())
                        .map(|(fire_rate, class)| format!("{} {:>16}", bar((fire_rate.timer.percent() * 10.0).round() as i32, 10, 10), class))
                        .collect::<Vec<String>>();
                    display.resize_with(5, Default::default);
                    display
                },
                UINode::Upgrades => {
                    let mut display = upgrades.display_for_ui();
                    display.resize_with(5, Default::default);
                    display
                }
            };

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
