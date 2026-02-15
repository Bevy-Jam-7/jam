use crate::gameplay::level::{CurrentLevel, EnvironmentAssets};
use crate::scatter::{components::*, layers::*};

use bevy::asset::{AssetEvent, Assets};
use bevy::image::Image;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_feronia::prelude::*;
use tracing::debug;

pub fn spawn_scatter_layers(mut cmd: Commands, landscape: Single<Entity, With<ScatterRoot>>) {
	let landscape = landscape.into_inner();
	debug!("Spawning scatter layers...");

	cmd.spawn((RockLayer, ChildOf(landscape)));
	cmd.spawn((MushroomLayer, ChildOf(landscape)));
	cmd.spawn((GrassLayer, ChildOf(landscape)));
}

pub fn clear_scatter_root(
	mut mw_clear_root: MessageWriter<ClearScatterRoot>,
	scatter_root: Single<Entity, With<ScatterRoot>>,
) {
	debug!("Clearing scatter root...");
	mw_clear_root.write((*scatter_root).into());
}

pub fn toggle_chunk_root(
	mut cmd: Commands,
	q_chunk_root: Query<Entity, With<ChunkRoot>>,
	current_level: Res<CurrentLevel>,
) {
	let enabled = matches!(
		*current_level,
		CurrentLevel::Commune | CurrentLevel::Shaders
	);
	toggle::<ChunkRootDisabled>(&mut cmd, q_chunk_root.iter(), enabled);
}

pub fn toggle_grass_layer(
	mut cmd: Commands,
	q_layer: Query<Entity, With<GrassLayer>>,
	current_level: Res<CurrentLevel>,
) {
	let enabled = matches!(
		*current_level,
		CurrentLevel::Commune | CurrentLevel::Shaders
	);

	toggle::<ScatterLayerDisabled>(&mut cmd, q_layer.iter(), enabled);
}

pub fn toggle_mushroom_layer(
	mut cmd: Commands,
	q_layer: Query<Entity, With<MushroomLayer>>,
	current_level: Res<CurrentLevel>,
) {
	let enabled = matches!(
		*current_level,
		CurrentLevel::Commune | CurrentLevel::Shaders
	);

	toggle::<ScatterLayerDisabled>(&mut cmd, q_layer.iter(), enabled);
}

pub fn toggle_rock_layer(
	mut cmd: Commands,
	q_layer: Query<Entity, With<RockLayer>>,
	current_level: Res<CurrentLevel>,
) {
	let enabled = matches!(
		*current_level,
		CurrentLevel::Commune | CurrentLevel::Shaders | CurrentLevel::Karoline
	);

	toggle::<ScatterLayerDisabled>(&mut cmd, q_layer.iter(), enabled);
}

fn toggle<T: Default + Component>(
	cmd: &mut Commands,
	mut iter: impl Iterator<Item = Entity>,
	enabled: bool,
) {
	while let Some(e) = iter.next() {
		if enabled {
			cmd.entity(e).remove::<T>();
		} else {
			cmd.entity(e).insert(T::default());
		}
	}
}

pub fn advance_to_setup(
	mut ns_scatter: ResMut<NextState<ScatterState>>,
	mut ns_height_state: ResMut<NextState<HeightMapState>>,
) {
	ns_scatter.set(ScatterState::Setup);
	ns_height_state.set(HeightMapState::Setup);
}

pub fn scatter(
	mut cmd: Commands,
	root: Single<Entity, With<ScatterRoot>>,
	current_level: Res<CurrentLevel>,
) {
	match *current_level {
		CurrentLevel::Commune | CurrentLevel::Shaders => {
			debug!("Scattering...");
			cmd.trigger(Scatter::<StandardMaterial>::new(*root));
		}
		_ => {
			cmd.trigger(ScatterDone);
		}
	}
}

pub fn update_density_map(
	mut ev_asset: MessageReader<AssetEvent<Image>>,
	mut assets: ResMut<Assets<Image>>,
	mut level_assets: ResMut<EnvironmentAssets>,
) {
	for id in ev_asset.read().filter_map(|ev| {
		let AssetEvent::Modified { id, .. } = ev else {
			return None;
		};
		Some(id)
	}) {
		if *id == level_assets.grass_density_map.id() {
			level_assets.grass_density_map = assets.get_strong_handle(*id).unwrap();
		}
		if *id == level_assets.rock_density_map.id() {
			level_assets.rock_density_map = assets.get_strong_handle(*id).unwrap();
		}
		if *id == level_assets.mushroom_density_map.id() {
			level_assets.mushroom_density_map = assets.get_strong_handle(*id).unwrap();
		}
	}
}

pub fn spawn_scatter_root(mut cmd: Commands) {
	cmd.spawn((ScatterRoot::default(), ChunkRoot::default()));
}
