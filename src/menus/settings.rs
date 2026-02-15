//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use bevy::ecs::query::QueryFilter;
use bevy::window::PresentMode;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};
use bevy_ahoy::camera::CharacterControllerCameraOf;
use bevy_seedling::prelude::*;

use crate::ui_layout::RootWidget;
use crate::{
	audio::{MusicPool, perceptual::PerceptualVolumeConverter},
	gameplay::player::camera::WorldModelFov,
	menus::Menu,
	screens::Screen,
	theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
	app.init_resource::<VsyncSetting>();
	app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
	app.add_systems(
		Update,
		go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
	);

	app.add_systems(
		Update,
		(
			update_volume_label::<With<GlobalVolumeLabel>, With<MainBus>>,
			update_volume_label::<With<MusicVolumeLabel>, With<SamplerPool<MusicPool>>>,
			update_volume_label::<With<SfxVolumeLabel>, With<SoundEffectsBus>>,
			update_camera_sensitivity_label,
			update_camera_fov_label,
			update_vsync.run_if(resource_exists_and_changed::<VsyncSetting>),
			update_vsync_label,
		)
			.run_if(in_state(Menu::Settings)),
	);
}

fn spawn_settings_menu(mut commands: Commands) {
	commands.spawn((
		RootWidget,
		DespawnOnExit(Menu::Settings),
		DespawnOnExit(Screen::Gameplay),
		GlobalZIndex(2),
		children![
			widget::header("Settings"),
			(
				Name::new("Settings Grid"),
				Node {
					display: Display::Grid,
					row_gap: Px(10.0),
					column_gap: Px(30.0),
					grid_template_columns: RepeatedGridTrack::px(2, 400.0),
					..default()
				},
				children![
					// Audio
					(
						widget::label("Global Volume"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
					widget::plus_minus_bar(
						GlobalVolumeLabel,
						lower_volume::<With<MainBus>>,
						raise_volume::<With<MainBus>>
					),
					(
						widget::label("Music Volume"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
					widget::plus_minus_bar(
						MusicVolumeLabel,
						lower_volume::<With<SamplerPool<MusicPool>>>,
						raise_volume::<With<SamplerPool<MusicPool>>>
					),
					(
						widget::label("Sound Effects Volume"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
					widget::plus_minus_bar(
						SfxVolumeLabel,
						lower_volume::<With<SoundEffectsBus>>,
						raise_volume::<With<SoundEffectsBus>>
					),
					// Camera Sensitivity
					(
						widget::label("Camera Sensitivity"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
					widget::plus_minus_bar(
						CameraSensitivityLabel,
						lower_camera_sensitivity,
						raise_camera_sensitivity
					),
					// Camera FOV
					(
						widget::label("Camera FOV"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
					widget::plus_minus_bar(CameraFovLabel, lower_camera_fov, raise_camera_fov),
					// VSync
					(
						widget::label("VSync"),
						Node {
							justify_self: JustifySelf::End,
							..default()
						}
					),
				],
			),
			widget::button("Back", go_back_on_click),
		],
	));
}

#[derive(Resource, Reflect, Debug)]
struct VolumeTicks(usize);

impl VolumeTicks {
	fn increment(&mut self) {
		self.0 = Self::MAX_TICK_COUNT.min(self.0 + 1);
	}

	fn decrement(&mut self) {
		self.0 = self.0.saturating_sub(1);
	}

	fn fraction(&self) -> f32 {
		self.0 as f32 / Self::MAX_TICK_COUNT as f32
	}

	fn label(&self) -> String {
		let filled = "█".repeat(self.0);
		let empty = " ".repeat(VolumeTicks::MAX_TICK_COUNT - self.0);
		filled + &empty + "|"
	}

	/// How many ticks the volume slider supports
	const MAX_TICK_COUNT: usize = 20;
}

impl From<VolumeTicks> for Volume {
	fn from(value: VolumeTicks) -> Self {
		PerceptualVolumeConverter::default().to_volume(value.fraction())
	}
}

impl From<Volume> for VolumeTicks {
	fn from(value: Volume) -> Self {
		VolumeTicks(
			(PerceptualVolumeConverter::default().to_perceptual(value)
				* Self::MAX_TICK_COUNT as f32)
				.round() as usize,
		)
	}
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SfxVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct QualitySettingsLabel;

fn lower_volume<F: QueryFilter>(_on: On<Pointer<Click>>, mut volume: Single<&mut VolumeNode, F>) {
	let mut ticks = VolumeTicks::from(volume.volume);
	ticks.decrement();
	volume.volume = ticks.into();
}

fn raise_volume<F: QueryFilter>(_on: On<Pointer<Click>>, mut master: Single<&mut VolumeNode, F>) {
	let mut ticks = VolumeTicks::from(master.volume);
	ticks.increment();
	master.volume = ticks.into();
}

fn update_volume_label<F1, F2>(mut label: Single<&mut Text, F1>, master: Single<&VolumeNode, F2>)
where
	F1: QueryFilter,
	F2: QueryFilter,
{
	let ticks = VolumeTicks::from(master.volume);
	label.0 = ticks.label();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSensitivityLabel;

fn lower_camera_sensitivity(
	_on: On<Pointer<Click>>,
	cam: Single<(Entity, &CharacterControllerCameraOf)>,
	mut command: Commands,
) {
	let (entity, cam) = cam.into_inner();
	let mut cam = cam.clone();
	cam.mult -= 0.1;
	const MIN_SENSITIVITY: f32 = 0.1;
	cam.mult.x = cam.mult.x.max(MIN_SENSITIVITY);
	cam.mult.y = cam.mult.y.max(MIN_SENSITIVITY);
	command.entity(entity).insert(cam);
}

fn raise_camera_sensitivity(
	_on: On<Pointer<Click>>,
	cam: Single<(Entity, &CharacterControllerCameraOf)>,
	mut command: Commands,
) {
	let (entity, cam) = cam.into_inner();
	let mut cam = cam.clone();
	cam.mult += 0.1;
	const MAX_SENSITIVITY: f32 = 20.0;
	cam.mult.x = cam.mult.x.min(MAX_SENSITIVITY);
	cam.mult.y = cam.mult.y.min(MAX_SENSITIVITY);
	command.entity(entity).insert(cam);
}

fn update_camera_sensitivity_label(
	mut label: Single<&mut Text, With<CameraSensitivityLabel>>,
	camera_sensitivity: Single<&CharacterControllerCameraOf>,
) {
	label.0 = format!("{:.1}", camera_sensitivity.mult.x);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraFovLabel;

fn lower_camera_fov(_on: On<Pointer<Click>>, mut camera_fov: ResMut<WorldModelFov>) {
	camera_fov.0 -= 1.0;
	camera_fov.0 = camera_fov.0.max(45.0);
}

fn raise_camera_fov(_on: On<Pointer<Click>>, mut camera_fov: ResMut<WorldModelFov>) {
	camera_fov.0 += 1.0;
	camera_fov.0 = camera_fov.0.min(130.0);
}

fn update_camera_fov_label(
	mut label: Single<&mut Text, With<CameraFovLabel>>,
	camera_fov: Res<WorldModelFov>,
) {
	label.0 = format!("{:.1}", camera_fov.0);
}

#[derive(Resource, Reflect, Debug)]
struct VsyncSetting(bool);

impl Default for VsyncSetting {
	fn default() -> Self {
		Self(false)
	}
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct VsyncLabel;

fn update_vsync(mut window: Single<&mut Window>, setting: Res<VsyncSetting>) {
	window.present_mode = if setting.0 {
		PresentMode::AutoVsync
	} else {
		PresentMode::Mailbox
	};
}

fn update_vsync_label(mut label: Single<&mut Text, With<VsyncLabel>>, setting: Res<VsyncSetting>) {
	label.0 = if setting.0 { "On".into() } else { "Off".into() };
}

fn go_back_on_click(
	_on: On<Pointer<Click>>,
	screen: Res<State<Screen>>,
	mut next_menu: ResMut<NextState<Menu>>,
) {
	next_menu.set(if screen.get() == &Screen::Title {
		Menu::Main
	} else {
		Menu::Pause
	});
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
	next_menu.set(if screen.get() == &Screen::Title {
		Menu::Main
	} else {
		Menu::Pause
	});
}
