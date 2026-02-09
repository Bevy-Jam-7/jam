use avian3d::prelude::*;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_enhanced_input::prelude::Start;
use bevy_seedling::sample::{AudioSample, RandomPitch, SamplePlayer};
use bevy_shuffle_bag::ShuffleBag;

use crate::{
	RenderLayer,
	audio::SfxPool,
	gameplay::{
		player::{camera::PlayerCamera, input::EatObject},
		stomach::Stomach,
	},
	third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
	app.add_observer(on_eat);
	app.add_observer(try_eat);

	app.init_resource::<EatSounds>()
		.init_resource::<GulpTimer>()
		.add_systems(Update, gulp)
		.add_observer(play_eat_sound);
}

/// Event for eating an entity and putting it into the stomach.
#[derive(EntityEvent, Debug)]
pub struct Eat {
	/// The rigid body entity to eat.
	#[event_target]
	pub body: Entity,
}

fn on_eat(
	eat: On<Eat>,
	mut transform_query: Query<&mut Transform>,
	mut layer_query: Query<(Option<&CollisionLayers>, Has<Mesh3d>)>,
	child_query: Query<&Children>,
	stomach: Single<(&mut Stomach, &GlobalTransform)>,
	mut commands: Commands,
) {
	let Ok(mut transform) = transform_query.get_mut(eat.body) else {
		return;
	};

	let (mut stomach, stomach_transform) = stomach.into_inner();

	// Move the entity to the stomach.
	// TODO: Spawn at the top?
	transform.translation = stomach_transform.translation();
	stomach.contents.insert(eat.body);

	// Lock the entity's Z translation.
	// TODO: Don't overwrite any other locked axes.
	commands
		.entity(eat.body)
		.insert(LockedAxes::default().lock_translation_z());

	// Change the collision and render layers to the stomach layers.
	for entity in std::iter::once(eat.body).chain(child_query.iter_descendants(eat.body)) {
		let Ok((collision_layers, has_mesh)) = layer_query.get_mut(entity) else {
			continue;
		};

		if let Some(collision_layers) = collision_layers {
			let mut new_layers = *collision_layers;
			new_layers.memberships.add(CollisionLayer::Stomach);
			new_layers.filters.add(CollisionLayer::Stomach);
			commands.entity(entity).insert(new_layers);
		}

		if has_mesh {
			commands
				.entity(entity)
				.insert(RenderLayers::from(RenderLayer::STOMACH));
		}
	}
}

fn try_eat(
	_eat: On<Start<EatObject>>,
	player: Single<&GlobalTransform, With<PlayerCamera>>,
	collider_of_query: Query<&ColliderOf>,
	spatial_query: SpatialQuery,
	mut commands: Commands,
) {
	const MAX_INTERACTION_DISTANCE: f32 = 3.0;

	let camera_transform = player.compute_transform();

	let hit = spatial_query.cast_ray(
		camera_transform.translation,
		camera_transform.forward(),
		MAX_INTERACTION_DISTANCE,
		true,
		&SpatialQueryFilter::from_mask(CollisionLayer::Prop),
	);

	if let Some(hit) = hit
		&& let Ok(collider_of) = collider_of_query.get(hit.entity)
	{
		commands.trigger(Eat {
			body: collider_of.body,
		});
	}
}

#[derive(Resource)]
struct EatSounds(ShuffleBag<Handle<AudioSample>>);

impl FromWorld for EatSounds {
	fn from_world(world: &mut World) -> Self {
		let assets = world.resource::<AssetServer>();
		let mut rng = rand::rng();

		Self(
			ShuffleBag::try_new(
				vec![
					assets.load("audio/sound_effects/mouth/eat1.ogg"),
					assets.load("audio/sound_effects/mouth/eat2.ogg"),
					assets.load("audio/sound_effects/mouth/eat3.ogg"),
				],
				&mut rng,
			)
			.unwrap(),
		)
	}
}

fn play_eat_sound(
	_: On<Eat>,
	mut gulp: ResMut<GulpTimer>,
	mut sounds: ResMut<EatSounds>,
	mut commands: Commands,
) {
	let rng = &mut rand::rng();
	let sound = sounds.0.pick(rng);

	gulp.timer = Some(Timer::from_seconds(0.5, TimerMode::Once));

	commands.spawn((
		SamplePlayer::new(sound.clone()),
		RandomPitch(1.05..1.25),
		SfxPool,
	));
}

#[derive(Resource)]
struct GulpTimer {
	timer: Option<Timer>,
	gulp: Handle<AudioSample>,
}

impl FromWorld for GulpTimer {
	fn from_world(world: &mut World) -> Self {
		let assets = world.resource::<AssetServer>();

		Self {
			timer: None,
			gulp: assets.load("audio/sound_effects/mouth/gulp.ogg"),
		}
	}
}

fn gulp(mut gulp: ResMut<GulpTimer>, time: Res<Time>, mut commands: Commands) {
	let Some(timer) = &mut gulp.timer else {
		return;
	};

	if timer.tick(time.delta()).is_finished() {
		gulp.timer = None;
		commands.spawn((
			SamplePlayer::new(gulp.gulp.clone()),
			RandomPitch::new(0.15),
			SfxPool,
		));
	}
}
