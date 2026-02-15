use crate::scatter::ScatterDone;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_feronia::prelude::*;

pub fn scatter_extended(
	_: On<ScatterFinished<StandardMaterial>>,
	mut cmd: Commands,
	q_scatter_root: Query<Entity, With<ScatterRoot>>,
) {
	debug!("Scattering Mushrooms...");
	for root in &q_scatter_root {
		cmd.trigger(Scatter::<ExtendedWindAffectedMaterial>::new(root));
	}
}

pub fn scatter_instanced(
	_: On<ScatterFinished<ExtendedWindAffectedMaterial>>,
	mut cmd: Commands,
	q_scatter_root: Query<Entity, With<ScatterRoot>>,
) {
	// Scatter the grass last so it doesn't grow on occupied areas.
	debug!("Scattering Grass...");
	for root in &q_scatter_root {
		cmd.trigger(Scatter::<InstancedWindAffectedMaterial>::new(root));
	}
}

pub fn scatter_done(_: On<ScatterFinished<InstancedWindAffectedMaterial>>, mut cmd: Commands) {
	cmd.trigger(ScatterDone);
}
