pub mod colour;

use std::{ops::{AddAssign}};

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, window::Window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(physics_system)
        .add_system(engine_system)
        .add_system(player_control)
        .run();
}

#[derive(Component)]
struct IsPlayer;

#[derive(Component)]
struct Physics {
    acceleration: Vec2,
    velocity: Vec2,
    pub drag: f32
}

impl Physics {
    fn add_force(&mut self, force: Vec2) -> () {
        self.acceleration.add_assign(force)
    }
}

#[derive(Component)]
struct Engine {
    pub target: Option<Vec2>,
    power: f32,
    speed: f32,
    max_speed: f32,
    depower_factor: f32
}

#[derive(Component)]
struct MainCamera;

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
            text: Text::from_section("o", text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: 100.0, y: 100.0, z: 0.0 }, scale: Vec3 { x: 0.5, y: 0.5, z: 1.0 }, ..default() },
            ..default()
        },
        IsPlayer,
        Physics { acceleration: Vec2 { x: 10.0, y: 0.0 }, velocity: Vec2 { x: 0.0, y: 0.0 }, drag: 5.0 },
        Engine { target: None, power: 15.0, speed: 0.0, max_speed: 50.0, depower_factor: 5.0 },
    ));
    // Spawn an enemy
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("w", text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: -100.0, y: -100.0, z: 0.0 }, scale: Vec3 { x: 0.5, y: 0.5, z: 1.0 }, ..default() },
            ..default()
        },
        Physics { acceleration: Vec2 { x: 10.0, y: 0.0 }, velocity: Vec2 { x: 0.0, y: 0.0 }, drag: 5.0 },
        Engine { target: None, power: 15.0, speed: 0.0, max_speed: 50.0, depower_factor: 5.0 },
    ));
}

fn physics_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Physics), (With<Transform>, With<Physics>)>,
) {
    for (mut transform, mut physics) in &mut query {
        // Not sure how to avoid cloning here
        let current_acceleration = physics.acceleration.clone();
        let drag = physics.drag.clone();
        physics.velocity += current_acceleration;
        transform.translation += physics.velocity.extend(0.0) * time.delta_seconds();
        // TODO make acceleration ramp down
        physics.acceleration = Vec2::ZERO;
        physics.velocity *= 1.0 - (drag * time.delta_seconds());
    }
}

fn engine_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Physics, &mut Engine), (With<Transform>, With<Physics>, With<Engine>)>,
) {
    for (transform, mut physics, mut engine) in &mut query {
        let current = transform.translation.truncate();
        if let Some(target) = engine.target {
            engine.speed += engine.power * time.delta_seconds();
            if engine.speed > engine.max_speed { engine.speed = engine.max_speed; }
            let to_target = (target - current).normalize() * engine.speed;
            physics.add_force(to_target);
        } else {
            engine.speed -= engine.power * time.delta_seconds() * engine.depower_factor;
            if engine.speed < 0.0 { engine.speed = 0.0 }
        }
    }
}

fn player_control(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&IsPlayer, &mut Engine), (With<IsPlayer>, With<Engine>)>,
) {
    for (_, mut engine) in &mut query {
        if mouse_button_input.pressed(MouseButton::Left) {
            // Calculate current position to mouse position
            let (camera, camera_transform) = camera_q.single();
            let window = windows.get_single().expect("no primary window");

            engine.target = window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate());
        } else {
            engine.target = None;
        }
    }
}

