mod colour;
mod component;
mod math;
mod system;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_prototype_lyon::prelude::*;
use component::*;
use std::f32::consts::PI;
use system::*;
use rand::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Outer Shell"),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_spawners)
        .add_startup_system(debug_setup)
        .add_system(ui_system)
        .add_system(physics_system)
        .add_system(engine_system)
        .add_system(player_control)
        .add_system(camera_follow)
        .add_system(turret_system)
        .add_system(bullet_system)
        .add_system(bullet_collision_system)
        .add_system(combat_system)
        .add_system(spawner_system)
        .add_system(ai_system)
        .add_system(laser_render_system)
        .add_system(explosion_render_system)
        .add_system(death_system)
        .add_system(loot_magnet_system)
        .add_system(loot_cargo_collision)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the Camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(colour::BLACK),
            },
            ..Default::default()
        },
        MainCamera,
    ));
    // Spawn the player
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    "V",
                    TextStyle {
                        font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                        font_size: 40.0,
                        color: colour::PLAYER,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform {
                    translation: Vec3 {
                        x: 100.0,
                        y: 100.0,
                        z: 0.0,
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
            Engine::new(10.0, 20.0),
            Health::new(100, 100),
            Collider { radius: 5.0 },
            Targettable(Allegiance::PLAYER),
            WillTarget(vec![Allegiance::ENEMY]),
            Cargo::new(),
            Magnet { range: 500.0, strength: 5.0 },
        ))
        .with_children(|parent| {
            parent.spawn(Turret::blast_laser());
        });

    // UI
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
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Health",
                TextStyle {
                    font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                    font_size: 12.0,
                    color: colour::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Shield",
                TextStyle {
                    font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                    font_size: 12.0,
                    color: colour::SHIELD,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Engine",
                TextStyle {
                    font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                    font_size: 12.0,
                    color: colour::INACTIVE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Cargo",
                TextStyle {
                    font: asset_server.load("fonts/AnonymousPro-Regular.ttf"),
                    font_size: 12.0,
                    color: colour::RED,
                },
            ));
        });
}

fn spawn_spawners(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    // Spawn enemy spawners
    for _ in 0..10 {
        commands.spawn((
            Spawner::new(30.0, 2.0),
            Transform {
                translation: Vec3 {
                    x: rng.gen_range(-1000.0..1000.0),
                    y: rng.gen_range(-1000.0..1000.0),
                    z: 0.0,
                },
                ..default()
            },
        ));
    }
}

fn debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
