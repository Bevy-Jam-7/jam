use bevy::prelude::*;

use crate::screens::Screen;

pub(crate) mod ui;

pub(super) fn plugin(app: &mut App) {
	app.add_plugins(ui::plugin);

	app.add_systems(OnEnter(Screen::Gameplay), spawn_test_objectives);
}

/// A game objective.
#[derive(Component, Debug, Default)]
pub struct Objective {
	/// The description of the objective.
	pub description: String,
}

impl Objective {
	/// Creates a new [`Objective`] with the given description.
	pub fn new(description: impl Into<String>) -> Self {
		Self {
			description: description.into(),
		}
	}
}

/// Marker component for completed objectives.
#[derive(Component, Debug, Default)]
pub struct ObjectiveCompleted;

/// A relationship component linking a sub-objective to its parent objective.
#[derive(Component, Debug)]
#[relationship(relationship_target = SubObjectives)]
pub struct SubObjectiveOf {
	/// The parent objective entity.
	#[relationship]
	pub objective: Entity,
}

/// A relationship target component holding all sub-objectives of a parent objective.
#[derive(Component, Debug, Default, Deref)]
#[relationship_target(relationship = SubObjectiveOf)]
pub struct SubObjectives(Vec<Entity>);

fn spawn_test_objectives(mut commands: Commands) {
	// Spawn a top-level objective.
	commands.spawn((
		Objective::new("Task 1"),
		related!(SubObjectives[
			Objective::new("Task 1.1"),
			Objective::new("Task 1.2"),
			(Objective::new("Task 1.3"), ObjectiveCompleted)
		]),
	));

	commands.spawn((
		Objective::new("Task 2"),
		related!(SubObjectives[
			(Objective::new("Task 2.1"), ObjectiveCompleted),
			(
				Objective::new("Task 2.2"),
				related!(SubObjectives[
					Objective::new("Task 2.2.1"),
					Objective::new("Task 2.2.2"),
				]),
			),
			Objective::new("Task 2.3")
		]),
	));
}
