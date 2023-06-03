use bevy::prelude::*;
use rand::Rng;

use crate::component::TurretClass;

#[derive(Resource)]
pub struct VolumeOption(f32);

#[derive(Copy, Clone)]
pub enum SoundEffectEvent {
    WeaponFire(TurretClass),
    Explosion(f32),
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(VolumeOption(1.0))
            .add_system(sound_effect_events)
            .add_event::<SoundEffectEvent>();
    }
}

fn sound_effect_events(
    mut sound_effect_event: EventReader<SoundEffectEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    volume: Res<VolumeOption>,
) {
    for ev in sound_effect_event.iter() {
        match ev {
            SoundEffectEvent::Explosion(size) => {
                let speed = rand::thread_rng().gen_range(1.3..1.5) - (size / 80.0).clamp(0.0, 1.0);
                audio.play_with_settings(asset_server.load("sounds/explosion_01.wav"), PlaybackSettings { volume: volume.0, speed, ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::BlastLaser) => {
                audio.play_with_settings(asset_server.load("sounds/laser_01.wav"), PlaybackSettings { volume: volume.0 * 0.4, speed: rand::thread_rng().gen_range(0.7..1.0), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::AutoCannon) => {
                audio.play_with_settings(asset_server.load("sounds/projectile_01.wav"), PlaybackSettings { volume: volume.0 * 0.6, speed: rand::thread_rng().gen_range(0.7..1.0), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::PierceLaser) => {
                audio.play_with_settings(asset_server.load("sounds/laser_01.wav"), PlaybackSettings { volume: volume.0 * 0.6, speed: rand::thread_rng().gen_range(0.2..0.5), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::ChainLaser) => {
                audio.play_with_settings(asset_server.load("sounds/laser_02.wav"), PlaybackSettings { volume: volume.0 * 0.6, speed: rand::thread_rng().gen_range(1.0..1.2), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::ShrapnelCannon) => {
                audio.play_with_settings(asset_server.load("sounds/projectile_02.wav"), PlaybackSettings { volume: volume.0  * 0.6, speed: rand::thread_rng().gen_range(0.9..1.1), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(TurretClass::Emp) => {
                audio.play_with_settings(asset_server.load("sounds/emp_01.wav"), PlaybackSettings { volume: volume.0 * 0.4, speed: rand::thread_rng().gen_range(0.8..1.2), ..Default::default() });
            },
            SoundEffectEvent::WeaponFire(_) => (),
        };
    }
}

