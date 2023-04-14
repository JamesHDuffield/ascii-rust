pub mod colour;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_rotation)
        .run();
}

#[derive(Component)]
struct AnimateRotation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/AnonymousPro-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: colour::PLAYER,
    };
    // Spawn the Camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(colour::BLACK),
        },
        ..Default::default()
    });
    // Spawn the player
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("V", text_style.clone()).with_alignment(TextAlignment::Center),
            transform: Transform { translation: Vec3 { x: 100.0, y: 100.0, z: 0.0 }, scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, ..default() },
            ..default()
        },
        AnimateRotation,
    ));
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}

