use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use rand::{thread_rng, Rng};

use crate::*;

#[derive(Component, Copy, Clone)]
pub struct SpaceObject;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup_space_objects);
    }
}

pub fn generate_object_geometry(sides: i32, min_radius: f32, max_radius: f32) -> Path {
    let mut rng = rand::thread_rng();
    let mut path_builder = PathBuilder::new();
    let step: f32 = 2. * PI / sides as f32;
    path_builder.move_to(Vec2::from_angle(0.) * rng.gen_range(min_radius..max_radius));
    for n in 1..sides+1 {
        let angle: f32 = step * n as f32;
        path_builder.line_to(Vec2::from_angle(angle) * rng.gen_range(min_radius..max_radius));
    }
    path_builder.close();
    return path_builder.build();
  }

fn spawn_space_object(commands: &mut Commands) {
    let mut rng = thread_rng();
    let position = util::Math::random_2d_unit_vector() * 500.0;
    let size: f32 = rng.gen_range(20.0..40.0);
    commands
        .spawn((
            SpaceObject,
            Collider { radius: size },
            Physics { velocity: util::Math::random_2d_unit_vector() * rng.gen_range(3.0..8.0), face_velocity: false, ..Default::default()},
            Rotator { speed: rng.gen_range(-0.4..0.4) },
            Health::new(50, 0),
            Stroke::new(Colour::WHITE, 2.0),
            ShapeBundle {
                path: generate_object_geometry(10, size-10., size+10.),
                spatial: SpatialBundle::from_transform(Transform::from_translation(position.extend(RenderLayer::Background.as_z()))),
                ..default()
            },
            ExplodesOnDespawn {
                size_min: 50.0,
                size_max: 100.0,
                ..Default::default()
            },
        ));
}

fn setup_space_objects(mut commands: Commands) {
    for _ in 0..8 {
        spawn_space_object(&mut commands);
    }
}