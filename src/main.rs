#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;
mod component;
mod plugin;
mod resource;
mod system;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_parallax::{LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxPlugin, ParallaxResource, ParallaxSystems};
use bevy_prototype_lyon::prelude::*;
use component::*;
use plugin::EnemyPlugin;
use plugin::HudPlugin;
use plugin::TurretPlugin;
use plugin::UpgradePlugin;
use plugin::MainMenuPlugin;
use plugin::SelectionPlugin;
use util::RenderLayer;
use util::Colour;
use resource::*;
use std::f32::consts::PI;
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
    Selection,
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
        .add_plugin(SelectionPlugin)
        .add_plugin(UpgradePlugin)
        .add_plugin(TurretPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(EnemyPlugin)
        // InGame
        .add_system(
            setup_player.in_schedule(OnEnter(AppState::InGame)),
        )
        // Always run while game is running
        .add_system(pause_control.in_set(OnUpdate(AppState::InGame)))
        // Only run when unpaused
        .add_systems(
            (
                physics_system,
                engine_system,
                player_control,
                camera_follow.before(ParallaxSystems),
                bullet_system,
                bullet_collision_system,
                combat_system,
                laser_render_system,
                explosion_render_system,
                death_system,
                loot_magnet_system,
                loot_cargo_collision,
                seeker_system,
                level_up_system,
            )
                .distributive_run_if(game_not_paused)
                .in_set(OnUpdate(AppState::InGame)),
        )
        // Cleanup
        .add_system(reset_game.in_schedule(OnExit(AppState::InGame)))
        // Resources required on boot
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
    game_state.0 != GameState::Paused && game_state.0 != GameState::Selection
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

    // Start player at level 0 so they get immediate selection
    commands.insert_resource(PlayerLevel { value: 0 });

    // Spawn the Camera
    commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Colour::BLACK),
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
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "V",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 20.0,
                            color: Colour::PLAYER,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(Vec3 {
                        x: 100.0,
                        y: 100.0,
                        z: RenderLayer::Player.as_z(),
                    }),
                    ..default()
                },
                physics: Physics::new(5.0),
                engine: Engine::new_with_steering(8.0, 16.0, 10.0),
                health: Health::new(100, 100),
                collider: Collider { radius: 10.0 },
                targettable: Targettable(Allegiance::PLAYER),
                will_target: WillTarget(vec![Allegiance::ENEMY]),
                ..Default::default()
            },
            
            BaseGlyphRotation {
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            IsPlayer,
            Cargo::default(),
            Magnet::default(),
        ));
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
