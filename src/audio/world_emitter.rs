use bevy::{platform::collections::HashMap, prelude::*};
use bevy_seedling::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::audio::{SpatialPool, doppler::DopplerSound};

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<SoundMap>()
			.add_observer(observe_world_emitter);
	}
}

#[point_class(base(Transform, Visibility))]
pub struct WorldEmitter {
	source: WorldSounds,
	// volume in decibels
	volume: f32,
	// the unit scale is a bit unintuitive -- it sets
	// the scale of the units, meaning larger values result
	// in smaller sound radii
	unit_scale: f32,
}

impl Default for WorldEmitter {
	fn default() -> Self {
		Self {
			source: WorldSounds::Computer,
			volume: 0.0,
			unit_scale: 4.0,
		}
	}
}

#[derive(Resource)]
struct SoundMap(HashMap<WorldSounds, Handle<AudioSample>>);

impl FromWorld for SoundMap {
	fn from_world(world: &mut World) -> Self {
		let map = HashMap::from_iter([
			(
				WorldSounds::Computer,
				world.load_asset("audio/sound_effects/office/computer.ogg"),
			),
			(
				WorldSounds::Light1,
				world.load_asset("audio/sound_effects/office/fluorescent-light-1.ogg"),
			),
			(
				WorldSounds::Light2,
				world.load_asset("audio/sound_effects/office/fluorescent-light-2.ogg"),
			),
		]);

		Self(map)
	}
}

#[derive(PartialEq, Eq, Hash, Reflect, FgdType)]
enum WorldSounds {
	Computer,
	Light1,
	Light2,
}

fn observe_world_emitter(
	trigger: On<Insert, WorldEmitter>,
	emitter: Query<&WorldEmitter>,
	map: Res<SoundMap>,
	mut commands: Commands,
) -> Result {
	let emitter = emitter.get(trigger.entity)?;
	let sound = map
		.0
		.get(&emitter.source)
		.ok_or("Failed to find world sound")?;

	commands.entity(trigger.entity).insert((
		SamplePlayer::new(sound.clone())
			.looping()
			.with_volume(Volume::Decibels(emitter.volume)),
		PlaybackSettings::default().remove(),
		DopplerSound { strength: 0.5 },
		SpatialPool,
		RandomPitch::new(0.04),
		sample_effects![(
			SpatialBasicNode::default(),
			SpatialScale(Vec3::splat(emitter.unit_scale))
		)],
	));

	Ok(())
}
