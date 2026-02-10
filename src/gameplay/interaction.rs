use crate::gameplay::player::input::Interact;
use avian3d::prelude::PhysicsPickable;
use bevy::prelude::*;
use bevy::{ecs::component::Component, picking::Pickable};
use bevy_enhanced_input::prelude::Start;

/// Marker component for an entity being interactable by clicking on it.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Pickable, PhysicsPickable)]
pub struct InteractableObject(pub Option<String>);

/// [`Resource`] describing whether there is an interactable action available and optionally if there is a name for it.
#[derive(Resource, Default)]
pub struct AvailableInteraction {
	pub target_entity: Option<Entity>,
	pub description: Option<String>,
}

/// [`Event`] triggered when the specified entity was interacted with.
#[derive(Event)]
pub struct InteractEvent(pub Entity);

pub(super) fn plugin(app: &mut App) {
	app.init_resource::<AvailableInteraction>()
		.add_observer(picking_click_observer)
		.add_observer(interact_by_input_action)
		.add_observer(watch_for_interactability_hover)
		.add_observer(watch_for_interactability_unhover);
}

fn picking_click_observer(
	mut trigger: On<Pointer<Click>>,
	interaction_query: Query<(), With<InteractableObject>>,
	mut commands: Commands,
) {
	if interaction_query.contains(trigger.entity) {
		trigger.propagate(false);
		commands.trigger(InteractEvent(trigger.entity));
	}
}

fn interact_by_input_action(
	_on: On<Start<Interact>>,
	focused_object: Res<AvailableInteraction>,
	interaction_query: Query<(), With<InteractableObject>>,
	mut commands: Commands,
) {
	if let Some(entity) = focused_object.target_entity {
		if interaction_query.contains(entity) {
			commands.trigger(InteractEvent(entity));
		}
	}
}

fn watch_for_interactability_hover(
	trigger: On<Pointer<Over>>,
	interaction_query: Query<&InteractableObject>,
	mut resource: ResMut<AvailableInteraction>,
) {
	if let Ok(interaction) = interaction_query.get(trigger.entity) {
		resource.target_entity = Some(trigger.entity);
		resource.description = interaction.0.clone();
	}
}

fn watch_for_interactability_unhover(
	trigger: On<Pointer<Out>>,
	mut resource: ResMut<AvailableInteraction>,
) {
	if let Some(entity) = resource.target_entity {
		if entity == trigger.entity {
			resource.target_entity = None;
			resource.description = None;
		}
	}
}
