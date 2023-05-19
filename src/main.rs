#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod colour;
mod component;
mod layer;
mod math;
mod menu;
mod resource;
mod system;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_parallax::{LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxPlugin, ParallaxResource, ParallaxSystems};
use bevy_prototype_lyon::prelude::*;
use component::*;
use layer::RenderLayer;
use menu::MainMenuPlugin;
use resource::*;
use std::{f32::consts::PI, time::Duration};
use system::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Outer Shell"),
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(ShapePlugin)
        .add_plugin(ParallaxPlugin)
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_plugin(MainMenuPlugin)
        // InGame
        .add_systems(
            (setup_player, setup_hud).in_schedule(OnEnter(AppState::InGame)),
        )
        // Always run while game is running
        .add_systems((ui_system, pause_control).in_set(OnUpdate(AppState::InGame)))
        // Only run when unpaused
        .add_systems(
            (
                physics_system,
                engine_system,
                player_control,
                camera_follow.before(ParallaxSystems),
                turret_system,
                bullet_system,
                bullet_collision_system,
                combat_system,
                laser_render_system,
                explosion_render_system,
                ai_system,
                death_system,
                loot_magnet_system,
                loot_cargo_collision,
                seeker_system,
            )
                .distributive_run_if(game_not_paused)
                .in_set(OnUpdate(AppState::InGame)),
        )
        // Stop when game over
        .add_system(
            spawner_system
                .run_if(in_state(GameState::Running))
                .in_set(OnUpdate(AppState::InGame)),
        )
        // Cleanup
        .add_system(reset_game.in_schedule(OnExit(AppState::InGame)))
        .insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.95, 0.95),
                    path: "nebula-tile.png".to_string(),
                    tile_size: Vec2::new(1024.0, 1024.0),
                    scale: 5.0,
                    z: RenderLayer::Background.as_z_with_offset(-10.),
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.9, 0.9),
                    path: "stars-tile.png".to_string(),
                    tile_size: Vec2::new(1024.0, 1024.0),
                    scale: 1.0,
                    z: RenderLayer::Background.as_z(),
                    ..default()
                },
            ],
            ..default()
        })
        .run();
}

fn game_not_paused(game_state: Res<State<GameState>>) -> bool {
    game_state.0 != GameState::Paused
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    // Create resources
    commands.insert_resource(Fonts {
        primary: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
    });

    // Set the start time
    commands.insert_resource(GameTime { start_time: time.elapsed().clone() });

    // Create point count
    commands.insert_resource(Points { value: 0 });

    // Set spawn limit
    let seconds = 30.0;
    let mut timer = Timer::from_seconds(seconds, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs_f32(seconds - 1.0));
    commands.insert_resource(Spawning { max: 100, timer });

    // Spawn the Camera
    commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(colour::BLACK),
                },
                ..Default::default()
            },
            MainCamera,
        ))
        .insert(ParallaxCameraComponent);
    // Spawn a shape so that the shape loop always runs (fixes bug with library cleaning itself up)
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
            ..default()
        },
    ));
}

// Spawn the player
fn setup_player(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    "V",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 40.0,
                        color: colour::PLAYER,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform {
                    translation: Vec3 {
                        x: 100.0,
                        y: 100.0,
                        z: RenderLayer::Player.as_z(),
                    },
                    scale: Vec3 {
                        x: 0.5,
                        y: 0.5,
                        z: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            IsPlayer,
            Physics::new(5.0),
            Engine::new_with_steering(20.0, 40.0, 10.0),
            Health::new(100, 100),
            Collider { radius: 5.0 },
            Targettable(Allegiance::PLAYER),
            WillTarget(vec![Allegiance::ENEMY]),
            Cargo::new(),
            Magnet {
                range: 500.0,
                strength: 5.0,
            },
            ExplodesOnDespawn::default(),
            DespawnWithScene,
        ))
        .with_children(|parent| {
            parent.spawn(Turret::mine_launcher());
        });
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
            UINode,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Health",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Shield",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::SHIELD,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Engine",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::INACTIVE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Cargo",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: colour::RED,
                },
            ));
        });
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<DespawnWithScene>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut points: ResMut<Points>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_game_state.set(GameState::Running);
    points.value = 0;
}
