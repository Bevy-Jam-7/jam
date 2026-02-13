use crate::gameplay::level::LevelAssets;

use bevy::prelude::*;
use bevy_eidolon::prelude::*;
use bevy_feronia::asset::backend::scene_backend::SceneAssetBackendPlugin;
use bevy_feronia::prelude::*;

pub(crate) use components::ScatterDone;

pub mod components;
pub mod layers;
mod observers;
mod systems;

use observers::*;
use systems::*;

pub fn plugin(app: &mut App) {
	app.add_plugins(ScatterPlugin);
}

pub struct ScatterPlugin;

impl Plugin for ScatterPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(GlobalWind {
			current: Wind {
				noise_scale: 0.005,
				..WindPreset::Normal.into()
			},
			..default()
		})
		.add_plugins((
			SceneAssetBackendPlugin,
			StandardScatterPlugin,
			InstancedWindAffectedScatterPlugin,
			ExtendedWindAffectedScatterPlugin,
			GpuComputeCullCorePlugin,
			GpuCullComputePlugin::<InstancedWindAffectedMaterial>::default(),
		));

		app.init_resource::<ScatterDone>();
		app.add_systems(OnEnter(HeightMapState::Ready), spawn_scatter_layers);
		app.add_systems(OnEnter(ScatterState::Ready), scatter);
		app.add_observer(scatter_extended);
		app.add_observer(scatter_instanced);

		app.add_systems(Startup, spawn_scatter_root);
		app.add_systems(
			Update,
			(
				scatter.run_if(resource_exists_and_changed::<LevelAssets>),
				update_density_map.run_if(resource_exists::<LevelAssets>),
			),
		);
	}
}
