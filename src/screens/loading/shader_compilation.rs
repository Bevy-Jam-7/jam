//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use super::LoadingScreen;
use crate::font::VARIABLE_FONT;
use crate::gameplay::level::AdvanceLevel;
use crate::theme::palette::HEADER_TEXT;
use crate::{
	shader_compilation::{LoadedPipelineCount, all_pipelines_loaded},
	theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
	app.add_systems(
		OnEnter(LoadingScreen::Shaders),
		spawn_or_skip_shader_compilation_loading_screen,
	);

	app.add_systems(
		Update,
		(
			update_loading_shaders_label,
			exit_shader_level.run_if(all_pipelines_loaded),
		)
			.chain()
			.run_if(in_state(LoadingScreen::Shaders))
			.run_if(resource_exists::<ShaderTimeout>),
	);
}

#[derive(Debug, Deref, DerefMut, Resource, Clone, Default)]
pub struct ShaderTimeout(Timer);

fn spawn_or_skip_shader_compilation_loading_screen(
	mut commands: Commands,
	loaded_pipeline_count: Res<LoadedPipelineCount>,
	mut next_screen: ResMut<NextState<LoadingScreen>>,
) {
	if loaded_pipeline_count.is_done() {
		next_screen.set(LoadingScreen::Level);
		return;
	}
	commands.spawn((
		widget::ui_root("Loading Screen"),
		DespawnOnExit(LoadingScreen::Shaders),
		children![(
			Name::new("Compiling shaders text"),
			Text("Compiling Shaders".into()),
			TextFont {
				font: VARIABLE_FONT,
				font_size: 24.0,
				weight: FontWeight(800),
				..default()
			},
			TextColor(HEADER_TEXT),
			LoadingShadersLabel
		)],
	));
	commands.insert_resource(ShaderTimeout(Timer::from_seconds(30.0, TimerMode::Once)))
}

fn exit_shader_level(mut cmd: Commands) {
	cmd.trigger(AdvanceLevel);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingShadersLabel;

fn update_loading_shaders_label(
	mut query: Query<&mut Text, With<LoadingShadersLabel>>,
	loaded_pipeline_count: Res<LoadedPipelineCount>,
	time: Res<Time<Real>>,
	mut shader_timeout: ResMut<ShaderTimeout>,
) {
	shader_timeout.tick(time.delta());

	for mut text in query.iter_mut() {
		text.0 = format!(
			"Compiling shaders: {} / {}\n(This may take up to 30 seconds, the fun begins soon!)",
			loaded_pipeline_count.0,
			LoadedPipelineCount::TOTAL_PIPELINES
		);
	}
}
