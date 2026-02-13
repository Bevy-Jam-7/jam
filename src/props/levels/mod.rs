use crate::audio::SpatialPool;
use crate::gameplay::TargetName;
use crate::gameplay::interaction::InteractEvent;
use crate::props::logic_entity::TimerEntity;
use crate::{gameplay::objectives::*, third_party::avian3d::CollisionLayer};
use avian3d::prelude::*;
use bevy::ecs::error::Result;
use bevy::prelude::*;
use bevy_seedling::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::{gameplay::level::LevelAssets, props::logic_entity::ObjectiveEntity};

pub(super) fn plugin(app: &mut App) {
	app.add_observer(setup_break_room);
}

fn setup_break_room(add: On<Add, BreakRoomSensor>, mut commands: Commands) -> Result {
	let entity = add.entity;
	commands
		.entity(entity)
		.insert((
			TargetName::new("break_room_sensor"),
			ObjectiveCompletor {
				target: "work_7".to_string(),
			},
			CollisionLayers::new([CollisionLayer::Sensor], [CollisionLayer::PlayerCharacter]),
		))
		.observe(tell_to_eat)
		.observe(kick_out);
	commands
		.spawn((
			Name::new("Objective: work_6".to_string()),
			TargetName::new("work_6"),
			ObjectiveEntity {
				target: None,
				objective_order: 6.0,
			},
			Objective::new("Increase Shareholder Value"),
			TimerEntity {
				timer_length: 5.0,
				timer_elapsed: 0.0,
				timer_on_finish: Some("break_room_sensor".to_string()),
				timer_active: false,
				timer_repeating: false,
			},
		))
		.observe(
			move |_add: On<Add, ObjectiveCompleted>,
			      mut timer: Query<&mut TimerEntity>|
			      -> Result {
				let mut timer = timer.get_mut(entity)?;

				timer.timer_active = true;
				Ok(())
			},
		);
	Ok(())
}

fn kick_out(
	trigger: On<InteractEvent>,
	mut commands: Commands,
	level_assets: Res<LevelAssets>,
	transform: Query<&GlobalTransform>,
) {
	let translation = transform
		.get(trigger.0)
		.map(|t| t.translation())
		.unwrap_or_default();
	commands.spawn((
		SamplePlayer {
			sample: level_assets.break_room_alarm.clone(),
			repeat_mode: RepeatMode::RepeatMultiple {
				num_times_to_repeat: 2,
			},
			..default()
		},
		SpatialPool,
		Transform::from_translation(translation),
	));
}

#[solid_class(base(Transform, Visibility))]
#[require(Sensor, CollisionEventsEnabled)]
pub(crate) struct BreakRoomSensor;

fn tell_to_eat(
	_collision: On<CollisionStart>,
	mut commands: Commands,
	objectives: Query<&TargetName>,
	current_objective: Res<CurrentObjective>,
) -> Result<(), BevyError> {
	let Some(current_objective) = **current_objective else {
		return Ok(());
	};
	let Ok(targetname) = objectives.get(current_objective) else {
		return Ok(());
	};

	if **targetname == "work_5" {
		commands
			.entity(current_objective)
			.insert(ObjectiveCompleted);
	}
	Ok(())
}
