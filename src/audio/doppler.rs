use bevy::{ecs::entity::EntityHashMap, prelude::*};
use bevy_seedling::prelude::*;

pub struct DopplerPlugin;

impl Plugin for DopplerPlugin {
	fn build(&self, app: &mut App) {
		// NOTE: remove for upstreaming
		app.register_required_components::<SpatialListener3D, DopplerListener>();

		app.add_systems(
			PostUpdate,
			(update_listeners, update_sources)
				.chain()
				.after(TransformSystems::Propagate),
		)
		.add_observer(remove_doppler);
	}
}

/// Applies the Doppler effect to a `SamplePlayer`.
///
/// The amount of shifting corresponds to the strength parameter,
/// where 1.0 matches the effect in air at sea-level assuming
/// position units in meters.
#[derive(Component, Reflect)]
pub struct DopplerSound {
	pub strength: f32,
}

impl Default for DopplerSound {
	fn default() -> Self {
		Self { strength: 1.0 }
	}
}

fn remove_doppler(trigger: On<Remove, DopplerSound>, listeners: Query<&mut DopplerListener>) {
	for mut listener in listeners {
		listener.sources.remove(&trigger.entity);
	}
}

/// Listener-specific doppler data.
///
/// The possibility of multiple listeners means we need to
/// track data for each listener.
#[derive(Component, Default)]
struct DopplerListener {
	sources: EntityHashMap<DopplerData>,
}

struct DopplerData {
	/// The relative position of the source over a few frames.
	positions: Vec<(f32, Timestamp)>,
	/// The maximum number of positions to keep in the buffer.
	max_positions: usize,
}

impl Default for DopplerData {
	fn default() -> Self {
		Self {
			positions: Vec::new(),
			max_positions: 3,
		}
	}
}

#[derive(Clone, Copy)]
struct Timestamp(f64);

impl DopplerData {
	fn push(&mut self, distance: f32, time: Timestamp) {
		self.positions.push((distance, time));
		if self.positions.len() > self.max_positions {
			self.positions.remove(0);
		}
	}

	fn speed(&self) -> Option<f32> {
		if self.positions.len() < 2 {
			return None;
		}

		let mut speed = 0.0;
		for pair in self.positions.windows(2) {
			let (a, ta) = pair[0];
			let (b, tb) = pair[1];

			let time_delta = tb.0 - ta.0;
			let distance_delta = a - b;

			let units_per_second = if time_delta <= 0.0 {
				0.0
			} else {
				distance_delta / time_delta as f32
			};

			speed += units_per_second;
		}

		Some(speed / (self.positions.len() - 1) as f32)
	}
}

fn update_listeners(
	sources: Query<(Entity, &GlobalTransform), With<DopplerSound>>,
	mut listeners: Query<(&GlobalTransform, &mut DopplerListener)>,
	time: Res<Time>,
) {
	let timestamp = Timestamp(time.elapsed_secs_f64());
	for (source_entity, source_trans) in sources {
		for (listener_trans, mut listener) in &mut listeners {
			let relative_position = source_trans.translation() - listener_trans.translation();
			let data = listener.sources.entry(source_entity).or_default();
			data.push(relative_position.length(), timestamp);
		}
	}
}

fn update_sources(
	sources: Query<(
		Entity,
		&GlobalTransform,
		&mut PlaybackSettings,
		&DopplerSound,
	)>,
	listeners: Query<(&GlobalTransform, &DopplerListener)>,
) {
	for (source_entity, source_trans, mut settings, strength) in sources {
		let translation = source_trans.translation();

		// Get the closest listener, which we'll use for doppler information.
		let Some((_, listener)) = listeners.iter().min_by(|a, b| {
			let dist_a = a.0.translation().distance_squared(translation);
			let dist_b = b.0.translation().distance_squared(translation);

			dist_a.total_cmp(&dist_b)
		}) else {
			continue;
		};

		let Some(units_per_second) = listener.sources.get(&source_entity).and_then(|d| d.speed())
		else {
			continue;
		};

		let speed_of_sound = 343.0 * strength.strength;
		let playback_speed = speed_of_sound / (speed_of_sound - units_per_second).max(0.1);

		settings.speed = playback_speed as f64;
	}
}
