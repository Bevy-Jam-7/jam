use std::f32::consts::{PI, TAU};

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_bae::prelude::*;
use bevy_landmass::{Archipelago3d, FromAgentRadius as _, PointSampleDistance3d};
use bevy_trenchbroom::prelude::*;
use rand::{Rng, rng};

use crate::{
	gameplay::{npc::ai::NpcWalkTargetOf, player::Player},
	third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
	app.add_systems(FixedUpdate, update_sensors.before(BaeSystems::ExecutePlan));
}

fn update_sensors(
	spatial: SpatialQuery,
	mut enemies: Query<(Entity, &GlobalTransform, &mut Props, &mut EnemyAiState)>,
	player: Single<&Transform, With<Player>>,
	time: Res<Time>,
) {
	let player_transform = player.into_inner();
	for (enemy, transform, mut props, mut state) in enemies.iter_mut() {
		state.walk_timer.tick(time.delta());
		if !props.get::<bool>("alert") {
			let dist_sq = transform
				.translation()
				.distance_squared(player_transform.translation);
			const MAX_DIST: f32 = 30.0;
			if dist_sq < MAX_DIST * MAX_DIST
				&& let Ok(dir) = Dir3::new(player_transform.translation - transform.translation())
				&& spatial
					.cast_ray(
						transform.translation(),
						dir,
						MAX_DIST,
						true,
						&SpatialQueryFilter::from_mask([
							CollisionLayer::Default,
							CollisionLayer::Prop,
						]),
					)
					.is_none()
			{
				props.set("alert", true);
			}
		}
		//if !props.get::<bool>("alert") {}
	}
}

pub(crate) fn enemy_htn() -> impl Bundle {
	(
		EnemyAiState::default(),
		Plan::new(),
		Select,
		tasks![(
			conditions![Condition::eq("alert", false)],
			Operator::new(walk_randomly),
		),],
	)
}

fn walk_randomly(
	In(input): In<OperatorInput>,
	mut transforms: Query<&Transform>,
	archipelago: Single<&Archipelago3d>,
	mut states: Query<&EnemyAiState>,
	spatial: SpatialQuery,
	mut commands: Commands,
) -> OperatorStatus {
	let Ok(state) = states.get_mut(input.entity) else {
		return OperatorStatus::Failure;
	};

	let Ok(transform) = transforms.get_mut(input.entity) else {
		return OperatorStatus::Failure;
	};

	if state.walk_timer.is_finished() {
		let yaw = rng().random_range(0.0..TAU);
		let dir = Dir3::new_unchecked(Vec3::NEG_Z.rotate_y(yaw));
		const MAX_WALK_DIST: f32 = 10.0;
		let target_dist = spatial
			.cast_ray(
				transform.translation,
				dir,
				MAX_WALK_DIST,
				true,
				&SpatialQueryFilter::from_mask([
					CollisionLayer::Default,
					CollisionLayer::PlayerCharacter,
					CollisionLayer::Prop,
				]),
			)
			.map_or(MAX_WALK_DIST, |hit| hit.distance);
		let target_pos = transform.translation + dir * target_dist;
		error!(?target_pos);
		let Ok(target) =
			archipelago.sample_point(target_pos, &PointSampleDistance3d::from_agent_radius(0.2))
		else {
			return OperatorStatus::Failure;
		};
		commands
			.entity(input.entity)
			.with_related::<NpcWalkTargetOf>(Transform::from_translation(target.point()));
	}
	OperatorStatus::Success
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct EnemyAiState {
	walk_timer: Timer,
}

impl Default for EnemyAiState {
	fn default() -> Self {
		Self {
			walk_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
		}
	}
}
