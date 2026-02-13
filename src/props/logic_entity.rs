use bevy::prelude::*;

use bevy_trenchbroom::prelude::*;

use crate::{
	PostPhysicsAppSystems,
	gameplay::{
		TargetName, TargetnameEntityIndex,
		interaction::InteractEvent,
		objectives::{Objective, SubObjectiveOf},
	},
	props::interactables::InteractableEntity,
	reflection::ReflAppExt,
};

pub(super) fn plugin(app: &mut App) {
	app.register_dynamic_component::<ObjectiveEntity>()
		.register_dynamic_component::<YarnNode>()
		.register_dynamic_component::<TimerEntity>()
		.add_observer(interact_timers)
		.add_observer(uninitialise_objectives)
		.add_observer(talk_ify_yarnnode)
		.add_systems(
			Update,
			(
				initialise_objectives,
				tick_timers.in_set(PostPhysicsAppSystems::TickTimers),
			),
		);
}

fn uninitialise_objectives(add: On<Insert, ObjectiveEntity>, mut commands: Commands) {
	commands.entity(add.entity).insert(UnitialisedObjective);
}

fn initialise_objectives(
	uninit_objectives: Populated<(Entity, &ObjectiveEntity), With<UnitialisedObjective>>,
	objectives: Query<(), With<Objective>>,
	entity_index: Res<TargetnameEntityIndex>,
	mut commands: Commands,
) {
	for (entity, obj) in uninit_objectives.iter() {
		if let Some(target) = obj.target.as_ref() {
			if let Some(&parent) = entity_index
				.get_entity_by_targetname(target)
				.iter()
				.find(|entity| objectives.contains(**entity))
			{
				commands
					.entity(entity)
					.remove::<UnitialisedObjective>()
					.insert(SubObjectiveOf { objective: parent });
			}
		} else {
			commands.entity(entity).remove::<UnitialisedObjective>();
		}
	}
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct UnitialisedObjective;

/// An entity describing the identity of an objective
#[point_class(base(TargetName, Objective))]
#[derive(Default)]
pub(crate) struct ObjectiveEntity {
	/// The objective, if any, that this is a subobjective of
	/// DO NOT MUTATE, IT WON'T UPDATE
	pub target: Option<String>,
	/// The ordering of the objective, bigger = later
	pub objective_order: f32,
}

/// An entity describing a dialogue node or a script
/// To activate the script, launch an [`InteractEvent`]
/// Either by having the entity itself be interactable, or by relaying the event.
/// ## See Also
/// [`InteractableEntity::interaction_relay`]
#[point_class(base(TargetName))]
#[derive(Eq, PartialEq, Clone, Debug)]
pub(crate) struct YarnNode {
	/// Title of the yarn script that should be executed when this node is interacted with.
	#[class(must_set)]
	pub(crate) yarn_node: String,
	/// Whether this node should avoid the restrictions placed upon dialogue. TODO!
	pub(crate) is_non_dialogue: bool,
}

impl Default for YarnNode {
	fn default() -> Self {
		Self {
			yarn_node: "".to_string(),
			is_non_dialogue: false,
		}
	}
}

fn talk_ify_yarnnode(
	on: On<Add, YarnNode>,
	interactable_query: Query<&InteractableEntity>,
	mut commands: Commands,
) {
	if let Ok(interaction) = interactable_query.get(on.entity) {
		commands
			.entity(on.entity)
			.insert(interaction.add_override("Talk"));
	}
}

/// An entity describing a timer which triggers [`InteractEvent`] after some time.
/// Can also be used as a timed relay.
#[point_class(base(TargetName))]
#[derive(PartialEq, Clone, Debug, Default)]
pub(crate) struct TimerEntity {
	/// How long this timer takes to finish (in seconds)
	pub timer_length: f32,
	/// How long this timer has already been going for
	pub timer_elapsed: f32,
	/// Entities to interact with upon timer completion
	pub timer_on_finish: Option<String>,
	/// Whether this timer is currently ticking, disabled upon completion, activated upon interaction
	pub timer_active: bool,
	/// Whether this timer should start again after it finishes
	pub timer_repeating: bool,
}

fn interact_timers(trigger: On<InteractEvent>, mut timer_query: Query<&mut TimerEntity>) {
	if let Ok(mut timer) = timer_query.get_mut(trigger.0) {
		timer.timer_active = true;
	}
}

fn tick_timers(
	mut timer_query: Query<&mut TimerEntity>,
	mut commands: Commands,
	entity_index: Res<TargetnameEntityIndex>,
	time: Res<Time>,
) {
	let dt = time.delta_secs();
	for mut timer in timer_query.iter_mut() {
		let mut timer_activated = false;
		if timer.timer_active {
			timer.timer_elapsed += dt;
			if timer.timer_elapsed >= timer.timer_length {
				timer_activated = true;
				if timer.timer_repeating {
					if timer.timer_length.is_sign_positive() && timer.timer_length.is_finite() {
						timer.timer_elapsed =
							f32::rem_euclid(timer.timer_elapsed, timer.timer_length);
						if timer.timer_elapsed >= timer.timer_length {
							timer.timer_elapsed = 0.0;
						}
					}
				} else {
					timer.timer_elapsed = 0.0;
					timer.timer_active = false;
				}
			}
		}
		if timer_activated {
			if let Some(target) = &timer.timer_on_finish {
				for &entity in entity_index.get_entity_by_targetname(target) {
					commands.trigger(InteractEvent(entity));
				}
			}
		}
	}
}
