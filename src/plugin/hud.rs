use bevy::prelude::*;

use crate::{resource::{Fonts, PlayerLevel, GameTime}, component::*, util::Colour, AppState};

use super::PlayerUpgrades;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                setup_hud.in_schedule(OnEnter(AppState::InGame)),
            )
            // Always run while game is running
            .add_system(hud_system.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
pub enum UINode {
    Status,
    Equipment,
    Upgrades,
}

// Spawn the hud
fn setup_hud(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                    },
                    margin: UiRect::all(Val::Px(5.0)),
                    gap: Size {
                        height: Val::Px(2.0),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            UINode::Status,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::SHIELD,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::RED,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::INACTIVE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::PLAYER,
                },
            ));
        });
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { right: Val::Px(0.0), ..Default::default() },
                    size: Size {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                    },
                    margin: UiRect::all(Val::Px(5.0)),
                    gap: Size {
                        height: Val::Px(2.0),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            UINode::Equipment,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            for _ in 0..10 {
                parent.spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 12.0,
                        color: Colour::WHITE,
                    },
                ));
            }
        });
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { right: Val::Px(0.0), bottom: Val::Px(0.0), ..Default::default() },
                    size: Size {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                    },
                    margin: UiRect::all(Val::Px(5.0)),
                    gap: Size {
                        height: Val::Px(2.0),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            UINode::Upgrades,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            for _ in 0..10 {
                parent.spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 12.0,
                        color: Colour::WHITE,
                    },
                ));
            }
        });
}

fn bar(current: i32, max: i32, width: i32) -> String {
    if max == 0 {
        return String::from(' ').repeat(width as usize);
    }
    let bars: usize = match (current.clamp(0, max) * width / max).try_into() {
      Ok(val) => val,
      Err(_) => 0,
    };
    format!("{}{}", String::from('|').repeat(bars), String::from('.').repeat(width as usize - bars))
}
  
pub fn hud_system(
    upgrades: Res<PlayerUpgrades>,
    player_query: Query<(&Engine, &Health, &Cargo, &Children), With<IsPlayer>>,
    turret_query: Query<(&FireRate, &TurretClass)>,
    mut query: Query<(&Children, &UINode)>,
    mut q_child: Query<&mut Text>,
    level: Res<PlayerLevel>,
    game_time: Res<GameTime>,
) {
    if let Ok((engine, health, cargo, turrets)) = player_query.get_single() {
        // Loop over children and update display values
        for (children, ui_node) in &mut query {

            let displays = match ui_node {
                UINode::Status => vec![
                    format!("{:<8} {} {}", "Armor", bar(health.health, health.max_health, 10), health.health),
                    format!("{:<8} {} {}", "Shield", bar(health.shield, health.max_shield, 10), health.shield),
                    format!("{:<8} {} {:0>2}", "Level", bar(cargo.amount as i32, level.required_cargo_to_level() as i32, 10), level.value),
                    format!("{:<8} {} m/s", "Speed", engine.speed.round()),
                    format!("{:<8} {:0>2}:{:0>2}", "Time", game_time.0.elapsed().as_secs() / 60, game_time.0.elapsed().as_secs() % 60),
                ],
                UINode::Equipment => { 
                    let mut display = turrets
                        .iter()
                        .map(|e| turret_query.get(*e))
                        .filter_map(|result| result.ok())
                        .map(|(fire_rate, class)| format!("{} {:>16}", bar((fire_rate.timer.percent() * 10.0).round() as i32, 10, 10), class))
                        .collect::<Vec<String>>();
                    display.resize_with(10, Default::default);
                    display
                },
                UINode::Upgrades => {
                    let mut display = upgrades.display_for_ui();
                    display.resize_with(10, Default::default);
                    display
                }
            };

            for (i, display) in displays.iter().enumerate() {
                if let Some(&child) = children.get(i) {
                    if let Ok(mut text) = q_child.get_mut(child) {
                        if let Some(section) = text.sections.get_mut(0) {
                            section.value = display.to_string();
                        }
                    }
                }
            }
        }
    }
}