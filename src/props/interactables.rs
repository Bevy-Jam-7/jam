use bevy::{
	ecs::{lifecycle::HookContext, world::DeferredWorld},
	prelude::*,
};

use bevy_trenchbroom::prelude::*;

use crate::{
	gameplay::{
		interaction::InteractableObject, objectives::ObjectiveCompletor, stomach::EdibleProp,
	},
	reflection::ReflAppExt,
};

pub(super) fn plugin(app: &mut App) {
	app.register_dynamic_component::<InteractableEntity>();
}

/// Trenchbroom component for designing entities that can be interacted with.
#[derive(Default, Clone)]
#[base_class]
#[component(on_insert = InteractableEntity::on_insert)]
pub struct InteractableEntity {
	/// Whether this entity should be
	is_edible: bool,
	/// Whether this entity should have a special line of text for being interacted with or it should be inferred from being edible.
	interaction_text_override: Option<String>,
	/// What objective, if any, should be completed by this name. Should be the `targetname` of said objective.
	completes_subobjective: Option<String>,
	/// What entity, if any, should additionally receive [`InteractEvent`](crate::gameplay::interaction::InteractEvent) when this one activates.
	interaction_relay: Option<String>,
}

#[expect(dead_code)]
impl InteractableEntity {
	pub fn on_insert(mut world: DeferredWorld, ctx: HookContext) {
		if world.is_scene_world() {
			return;
		}
		if let Some(values) = world.get::<InteractableEntity>(ctx.entity).cloned() {
			if let Some(override_text) = values.interaction_text_override {
				world
					.commands()
					.entity(ctx.entity)
					.insert_if_new(InteractableObject(Some(override_text)));
			}
			if values.is_edible {
				world
					.commands()
					.entity(ctx.entity)
					.insert_if_new(EdibleProp);
			} else {
				world.commands().entity(ctx.entity).remove::<EdibleProp>();
			}
			if let Some(objective_name) = values.completes_subobjective {
				world
					.commands()
					.entity(ctx.entity)
					.insert_if_new(ObjectiveCompletor {
						target: objective_name,
					});
			}
		}
	}

	pub fn get_is_edible(&self) -> bool {
		self.is_edible
	}

	pub fn get_interaction_text_override(&self) -> Option<&str> {
		self.interaction_text_override.as_deref()
	}

	pub fn get_completes_subobjective(&self) -> Option<&str> {
		self.completes_subobjective.as_deref()
	}

	pub fn get_interaction_relay(&self) -> Option<&str> {
		self.interaction_relay.as_deref()
	}
}
