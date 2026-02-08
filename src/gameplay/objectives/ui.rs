use bevy::prelude::*;

use crate::{
	gameplay::objectives::{Objective, ObjectiveCompleted, SubObjectiveOf, SubObjectives},
	screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
	app.add_observer(on_spawn_objective);

	app.add_systems(OnEnter(Screen::Gameplay), spawn_objective_ui);
	app.add_systems(Update, update_objective_description_ui);
	app.add_systems(PostUpdate, handle_completed_objectives);
}

/// The UI node that holds all objectives.
#[derive(Component, Debug)]
pub struct ObjectiveUi;

/// A UI node representing a single objective.
#[derive(Component, Debug)]
pub struct ObjectiveNode;

/// A UI node representing a list of sub-objectives under a parent objective.
#[derive(Component, Debug)]
pub struct SubObjectiveListNode;

/// Links an [`Objective`] to a specific UI node in the world.
#[derive(Component, Debug)]
pub struct ObjectiveOfNode {
	pub node: Entity,
}

/// Spawns the main objective UI node.
fn spawn_objective_ui(mut commands: Commands) {
	commands.spawn((
		ObjectiveUi,
		Node {
			position_type: PositionType::Absolute,
			top: Val::Px(10.0),
			left: Val::Px(10.0),
			padding: UiRect::all(Val::Px(10.0)),
			flex_direction: FlexDirection::Column,
			row_gap: Val::Px(5.0),
			..default()
		},
		BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
	));
}

/// Creates a UI node for an objective with the given description.
fn objective_node(description: impl Into<String>, depth: usize) -> impl Bundle {
	(
		ObjectiveNode,
		Node {
			padding: UiRect::all(Val::Px(2.0)),
			flex_direction: FlexDirection::Column,
			row_gap: Val::Px((5.0 - depth as f32 * 0.5).max(2.0)),
			..default()
		},
		children![(
			Text::new(description),
			TextFont::from_font_size((16.0 - depth as f32 * 2.0).max(10.0)),
		)],
	)
}

/// Creates a UI node for a list of objectives.
fn sub_objective_list_node(depth: usize) -> impl Bundle {
	(
		SubObjectiveListNode,
		Node {
			flex_direction: FlexDirection::Column,
			padding: UiRect::left(Val::Px(20.0)),
			row_gap: Val::Px((5.0 - depth as f32 * 0.5).max(2.0)),
			..default()
		},
	)
}

/// Spawns an objective UI node when an objective is spawned.
fn on_spawn_objective(
	add: On<Add, Objective>,
	objective_query: Query<(&Objective, Option<&SubObjectiveOf>)>,
	objective_node_query: Query<&ObjectiveOfNode>,
	objective_ui: Single<Entity, With<ObjectiveUi>>,
	sub_objective_query: Query<&SubObjectiveOf>,
	child_query: Query<&Children>,
	mut commands: Commands,
) {
	let Ok((objective, sub_objective_of)) = objective_query.get(add.entity) else {
		return;
	};

	if let Some(sub_objective_of) = sub_objective_of {
		// This is a sub-objective; find the parent objective's UI node.
		let Ok(parent_objective_node) = objective_node_query.get(sub_objective_of.objective) else {
			return;
		};

		// Get the children of the parent objective node.
		let Ok(children) = child_query.get(parent_objective_node.node) else {
			return;
		};

		// Determine the depth of this sub-objective in the hierarchy.
		let depth = sub_objective_query.iter_ancestors(add.entity).count();

		// Find or create the objective list node under the parent objective node.
		let objective_list_node_entity = if children.len() == 1 {
			// No objective list node yet; create one.
			commands
				.spawn((
					ChildOf(parent_objective_node.node),
					sub_objective_list_node(depth),
				))
				.id()
		} else {
			// There is already an objective list node; use it.
			children[1]
		};

		let objective_node_entity = commands
			.spawn((
				ChildOf(objective_list_node_entity),
				objective_node(objective.description.clone(), depth),
			))
			.id();

		commands.entity(add.entity).try_insert(ObjectiveOfNode {
			node: objective_node_entity,
		});
	} else {
		// This is a top-level objective; add it to the main objective UI.
		let objective_node_entity = commands
			.spawn((
				ChildOf(*objective_ui),
				objective_node(objective.description.clone(), 0),
			))
			.id();

		commands.entity(add.entity).try_insert(ObjectiveOfNode {
			node: objective_node_entity,
		});
	}
}

/// When a parent objective is completed:
/// 1. Mark the parent objective's UI text as completed with a strikethrough,
///    and remove the sub-objective.
/// 2. If this objective is a sub-objective of another objective, check if all
///    sibling sub-objectives are completed, and if so, mark the parent objective as completed.
// TODO (Jondolf): I wanted to handle this with an observer, but had problems where
//                 siblings of completed sub-objectives were not yet spawned, and thus it
//                 would incorrectly mark the parent objective as completed.
fn handle_completed_objectives(
	new_completed_query: Query<
		(&ObjectiveOfNode, Option<&SubObjectiveOf>),
		Added<ObjectiveCompleted>,
	>,
	sub_objectives_query: Query<&SubObjectives>,
	child_query: Query<&Children>,
	completed_query: Query<(), With<ObjectiveCompleted>>,
	mut commands: Commands,
) {
	for (objective_node, sub_objective_of) in new_completed_query.iter() {
		// Update the objective UI to show the objective as completed.
		if let Ok(children) = child_query.get(objective_node.node) {
			let text_entity = children[0];
			commands
				.entity(text_entity)
				.try_insert((Strikethrough, StrikethroughColor(Color::WHITE)));

			// Remove the sub-objectives from the world.
			if let Some(sub_objective_list) = children.get(1) {
				commands.entity(*sub_objective_list).despawn();
			}
		}

		// Check if this objective is a sub-objective of another objective.
		let Some(sub_objective_of) = sub_objective_of else {
			continue;
		};

		// Check if all sibling sub-objectives are completed.
		let parent_objective = sub_objective_of.objective;
		if let Ok(sub_objectives) = sub_objectives_query.get(parent_objective)
			&& completed_query.iter_many(sub_objectives.iter()).count() == sub_objectives.len()
		{
			// Mark the parent objective as completed.
			commands.entity(parent_objective).insert(ObjectiveCompleted);
		}
	}
}

/// Updates the objective description UI when an objective's description changes.
fn update_objective_description_ui(
	objectives: Query<(&Objective, &ObjectiveOfNode), Changed<Objective>>,
	mut text_query: Query<&mut Text>,
	child_query: Query<&Children>,
) {
	for (objective, objective_of_node) in objectives.iter() {
		let Ok(children) = child_query.get(objective_of_node.node) else {
			continue;
		};

		let Ok(mut text) = text_query.get_mut(children[0]) else {
			continue;
		};

		text.0 = objective.description.clone();
	}
}
