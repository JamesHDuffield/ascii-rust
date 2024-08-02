#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;
mod component;
mod plugin;
mod resource;
mod system;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_parallax::CreateParallaxEvent;
use bevy_parallax::{LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxPlugin, ParallaxSystems};
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
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin { mode: bevy_embedded_assets::PluginMode::AutoLoad }),
        )
        .add_plugins(ShapePlugin)
        .add_plugins(ParallaxPlugin)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(UpgradePlugin)
        .add_plugins(TurretPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(EnemyPlugin)
        .add_event::<TakeDamageEvent>()
        // InGame
        .add_systems(OnEnter(AppState::InGame), (setup_new_game, setup_player))
        // Always run while game is running
        .add_systems(Update, (pause_control, zoom_control).run_if(in_state(AppState::InGame)))
        // Only run when unpaused
        .add_systems(Update,
            (
                game_time_system,
                physics_system,
                engine_system,
                player_control,
                camera_follow.before(ParallaxSystems),
                bullet_system,
                bullet_collision_system,
                combat_system,
                laser_render_system,
                explosion_render_system,
                expanding_collider_system,
                death_system,
                loot_magnet_system,
                loot_cargo_collision,
                seeker_system,
            )
                .distributive_run_if(game_not_paused)
                .distributive_run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update,
            (
                level_up_system,
                take_damage_events,
                hit_flash_system,
                floating_text_system,
            )
                .distributive_run_if(game_not_paused)
                .distributive_run_if(in_state(AppState::InGame)),
        )
        // Cleanup
        .add_systems(OnExit(AppState::InGame), reset_game)
        .run();
}

fn game_not_paused(game_state: Res<State<GameState>>) -> bool {
    *game_state.get() != GameState::Paused && *game_state.get() != GameState::Selection
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    // Set the font
    commands.insert_resource(Fonts {
        primary: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
    });

    
    // Spawn the Camera
    let camera = commands
        .spawn((
            Camera2dBundle::default(),
            MainCamera,
            CameraShake::default(),
        ))
        .insert(ParallaxCameraComponent::default())
        .id();

    // Setup parralax
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.95, 0.95),
                path: "nebula-tile.png".to_string(),
                tile_size: Vec2::new(1024.0, 1024.0),
                scale: Vec2::splat(5.0),
                z: RenderLayer::Background.as_z_with_offset(-10.),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                path: "stars-tile.png".to_string(),
                tile_size: Vec2::new(1024.0, 1024.0),
                z: RenderLayer::Background.as_z(),
                ..default()
            },
        ],
        camera,
    });

    // Spawn a shape so that the shape loop always runs (fixes bug with library cleaning itself up)
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
            ..default()
        },
    ));
}

fn setup_new_game(mut commands: Commands) {
    // Set the start time
    commands.insert_resource(GameTime::default());

    // Create point count
    commands.insert_resource(Points { value: 0 });

    // Start player at level 0 so they get immediate selection
    commands.insert_resource(PlayerLevel { value: 0 });
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
                    .with_justify(JustifyText::Center),
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

fn game_time_system(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<DespawnWithScene>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_game_state.set(GameState::Running);
}
