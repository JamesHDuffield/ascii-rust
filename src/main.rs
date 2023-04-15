mod colour;
mod math;
mod component;
mod system;

use std::f32::consts::PI;
use component::*;
use system::*;
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(physics_system)
        .add_system(engine_system)
        .add_system(player_control)
        .add_system(camera_follow)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/AnonymousPro-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: colour::PLAYER,
    };
    // Spawn the Camera
    commands.spawn((Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(colour::BLACK),
        },
        ..Default::default()
    }, MainCamera));
    // Spawn the player
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("V", text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: 100.0, y: 100.0, z: 0.0 }, scale: Vec3 { x: 0.5, y: 0.5, z: 1.0 }, ..default() },
            ..default()
        },
        BaseGlyphRotation { rotation: Quat::from_rotation_z(PI / 2.0) },
        IsPlayer,
        Physics::new(5.0),
        Engine::new(15.0, 50.0),
    ));
    // Spawn an enemy
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("w", text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: -100.0, y: -100.0, z: 0.0 }, scale: Vec3 { x: 0.5, y: 0.5, z: 1.0 }, ..default() },
            ..default()
        },
        Physics::new(5.0),
        Engine::new(10.0, 40.0),
    ));
}
