//! Spawn the main level.

use crate::{
	asset_tracking::{LoadResource, ResourceHandles},
	audio::MusicPool,
	gameplay::{
		npc::NPC_RADIUS,
		objectives::{AllObjectivesDone, Objective},
	},
	props::logic_entity::ObjectiveEntity,
	screens::{Screen, loading::LoadingScreen},
};
use bevy::prelude::*;
use bevy_landmass::prelude::*;
use bevy_rerecast::prelude::*;
use bevy_seedling::prelude::*;
use bevy_seedling::sample::AudioSample;

use landmass_rerecast::{Island3dBundle, NavMeshHandle3d};

pub(super) fn plugin(app: &mut App) {
	app.load_resource::<LevelAssets>()
		.init_asset::<LevelTwoAssets>();

	app.add_observer(advance_level);
	app.init_resource::<CurrentLevel>();
}

#[derive(Resource, Reflect, Debug, Default, Copy, Clone)]
#[reflect(Resource)]
pub(crate) enum CurrentLevel {
	#[default]
	DayOne,
	DayTwo,
}

/// A system that spawns the main level.
pub(crate) fn spawn_level(
	mut commands: Commands,
	level_assets: Res<LevelAssets>,
	level_two_assets: Option<Res<LevelTwoAssets>>,
	current_level: Res<CurrentLevel>,
) {
	match *current_level {
		CurrentLevel::DayOne => {
			commands.spawn((
				Objective::new("Clock In"),
				ObjectiveEntity {
					targetname: "start_work".into(),
					..Default::default()
				},
			));

			commands.spawn((
				Name::new("Level"),
				SceneRoot(level_assets.level.clone()),
				DespawnOnExit(Screen::Gameplay),
				Level,
				children![(
					Name::new("Level Music"),
					SamplePlayer::new(level_assets.music.clone()).looping(),
					MusicPool
				)],
			));

			let archipelago = commands
				.spawn((
					Name::new("Main Level Archipelago"),
					DespawnOnExit(Screen::Gameplay),
					Archipelago3d::new(ArchipelagoOptions::from_agent_radius(NPC_RADIUS)),
				))
				.id();

			commands.spawn((
				Name::new("Main Level Island"),
				DespawnOnExit(Screen::Gameplay),
				Island3dBundle {
					island: Island,
					archipelago_ref: ArchipelagoRef3d::new(archipelago),
					nav_mesh: NavMeshHandle3d(level_assets.navmesh.clone()),
				},
			));
		}
		CurrentLevel::DayTwo => {
			commands.spawn((
				Objective::new("Clock In"),
				ObjectiveEntity {
					targetname: "start_work".into(),
					..Default::default()
				},
			));
			let level_two_assets = level_two_assets.expect("If we don't have level two assets when spawning level two, we're in deep shit. Sorry player, we bail here.");

			commands.spawn((
				Name::new("Level"),
				SceneRoot(level_two_assets.level.clone()),
				DespawnOnExit(Screen::Gameplay),
				Level,
				children![(
					Name::new("Level Music"),
					SamplePlayer::new(level_two_assets.music.clone()).looping(),
					MusicPool
				)],
			));

			let archipelago = commands
				.spawn((
					Name::new("Main Level Archipelago"),
					DespawnOnExit(Screen::Gameplay),
					Archipelago3d::new(ArchipelagoOptions::from_agent_radius(NPC_RADIUS)),
				))
				.id();

			commands.spawn((
				Name::new("Main Level Island"),
				DespawnOnExit(Screen::Gameplay),
				Island3dBundle {
					island: Island,
					archipelago_ref: ArchipelagoRef3d::new(archipelago),
					nav_mesh: NavMeshHandle3d(level_assets.navmesh.clone()),
				},
			));
		}
	}
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
	#[dependency]
	pub(crate) level: Handle<Scene>,
	#[dependency]
	pub(crate) navmesh: Handle<Navmesh>,
	#[dependency]
	pub(crate) music: Handle<AudioSample>,
	#[dependency]
	pub(crate) break_room_alarm: Handle<AudioSample>,
}

impl FromWorld for LevelAssets {
	fn from_world(world: &mut World) -> Self {
		let assets = world.resource::<AssetServer>();

		Self {
			// Our main level is inspired by the TheDarkMod fan mission [Volta I: The Stone](https://www.thedarkmod.com/missiondetails/?internalName=volta1_3)
			level: assets.load("maps/main/one/one.map#Scene"),
			// You can regenerate the navmesh by using `bevy_rerecast_editor`
			navmesh: assets.load("maps/main/one/one.nav"),
			music: assets.load("audio/music/corpo slop to eat your computer to.ogg"),
			break_room_alarm: assets.load("audio/sound_effects/mental_health_alarm.ogg"),
		}
	}
}

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelTwoAssets {
	#[dependency]
	pub(crate) level: Handle<Scene>,
	#[dependency]
	pub(crate) navmesh: Handle<Navmesh>,
	#[dependency]
	pub(crate) music: Handle<AudioSample>,
}
impl FromWorld for LevelTwoAssets {
	fn from_world(world: &mut World) -> Self {
		let assets = world.resource::<AssetServer>();

		Self {
			// Our main level is inspired by the TheDarkMod fan mission [Volta I: The Stone](https://www.thedarkmod.com/missiondetails/?internalName=volta1_3)
			level: assets.load("maps/main/two/two.map#Scene"),
			// You can regenerate the navmesh by using `bevy_rerecast_editor`
			navmesh: assets.load("maps/main/two/two.nav"),
			music: assets.load("audio/music/corpo slop to eat your computer to.ogg"),
		}
	}
}

fn advance_level(_done: On<AllObjectivesDone>, mut commands: Commands) {
	commands.queue(|world: &mut World| {
		let value = LevelTwoAssets::from_world(world);
		let assets = world.resource::<AssetServer>();
		let handle = assets.add(value);
		let mut handles = world.resource_mut::<ResourceHandles>();
		handles
			.waiting
			.push_back((handle.untyped(), move |world, handle| {
				let assets = world.resource::<Assets<LevelTwoAssets>>();
				if let Some(value) = assets.get(handle.id().typed::<LevelTwoAssets>()) {
					world.insert_resource(value.clone());
				}
			}));
		world
			.resource_mut::<NextState<LoadingScreen>>()
			.set(LoadingScreen::Assets);
		world
			.resource_mut::<NextState<Screen>>()
			.set(Screen::Loading);
		*world.resource_mut::<CurrentLevel>() = CurrentLevel::DayTwo;
	});
}
