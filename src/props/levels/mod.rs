use std::time::Duration;

use crate::gameplay::objectives::*;
use avian3d::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::ecs::{error::Result, query::QueryData};
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::{
	gameplay::level::LevelAssets,
	props::logic_entity::ObjectiveEntity,
	timer::{GenericTimer, TimerFinished},
};

pub(super) fn plugin(app: &mut App) {
	app.add_observer(setup_break_room);
}

fn setup_break_room(add: On<Add, BreakRoomSensor>, mut commands: Commands) {
	commands
		.entity(add.entity)
		.insert(
			GenericTimer::<BreakRoomTimer>::new(Timer::new(
				Duration::from_secs(3),
				TimerMode::Once,
			))
			.with_active(false),
		)
		.observe(tell_to_eat)
		.observe(deactivate_timer)
		.observe(eating_objective);
}

fn kick_out(
	_: On<TimerFinished<BreakRoomTimer>>,
	mut commands: Commands,
	objectives: Query<(Entity, &ObjectiveEntity)>,
	level_assets: Res<LevelAssets>,
) {
	if let Some((entity, _)) = objectives
		.iter()
		.find(|(_, ObjectiveEntity { targetname, .. })| targetname == "work_5")
	{
		commands.spawn((
			SamplePlayer {
				sample: level_assets.break_room_alarm.clone(),
				repeat_mode: RepeatMode::RepeatMultiple {
					num_times_to_repeat: 5,
				},
				..default()
			},
			SpatialPool,
			Transform::from_xyz(7.0, 21., -2.),
		));
		commands.entity(entity).insert(ObjectiveCompleted);
		commands.spawn((
			Objective::new("Break's over"),
			ObjectiveEntity {
				targetname: "back_to_llm".into(),
				..default()
			},
			related!(SubObjectives[Objective::new("Talk to LLManager")]),
		));
	}
}

struct BreakRoomTimer;

#[solid_class(base(Transform, Visibility))]
#[require(Sensor, CollisionEventsEnabled)]
pub(crate) struct BreakRoomSensor;

fn tell_to_eat(
	collision: On<CollisionStart>,
	mut commands: Commands,
	mut timer: Query<&mut GenericTimer<BreakRoomTimer>>,
	objectives: Query<&ObjectiveEntity>,
	current_objective: Res<CurrentObjective>,
) -> Result<(), BevyError> {
	let Some(current_objective) = **current_objective else {
		return Ok(());
	};
	let Ok(ObjectiveEntity { targetname, .. }) = objectives.get(current_objective) else {
		return Ok(());
	};

	if targetname == "work_5" {
		commands
			.entity(current_objective)
			.insert(ObjectiveCompleted);
	}
	Ok(())
}

fn tell_to_chill(mut timer: Query<&mut GenericTimer<BreakRoomTimer>>) {
	let mut timer = timer.get_mut(collision.collider1)?;

	timer.set_active(true);
}

fn deactivate_timer(
	collision: On<CollisionEnd>,
	mut timer: Query<&mut GenericTimer<BreakRoomTimer>>,
) -> Result<(), BevyError> {
	let mut timer = timer.get_mut(collision.collider1)?;

	timer.set_active(false);

	Ok(())
}

#[derive(SystemParam)]
struct ObjectiveQuery<'w, 's> {
	objectives: Query<'w, 's, (Entity, &'static ObjectiveEntity)>,
}

impl<'w, 's> ObjectiveQuery<'w, 's> {
	pub fn get_objective(&self, name: &str) -> Result<Entity> {
		self.objectives
			.iter()
			.find_map(|(entity, objective)| (objective.targetname == name).then_some(entity))
			.ok_or_else(|| format!("Failed to find objective {name}").into())
	}
}

pub(crate) fn complete_objective(name: &str) -> impl Command<Result> {
	let name = name.to_string();
	move |world: &mut World| -> Result {
		let entity = world
			.query::<(Entity, &ObjectiveEntity)>()
			.iter(world)
			.find_map(|(entity, objective)| (objective.targetname == name).then_some(entity))
			.ok_or_else(|| format!("Failed to find objective {name}"))?;

		world.entity_mut(entity).insert(ObjectiveCompleted);
		Ok(())
	}
}
