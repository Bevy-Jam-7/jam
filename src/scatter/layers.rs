use crate::gameplay::level::EnvironmentAssets;
use crate::props::effects::disable_shadow_casting_on_instance_ready;
use crate::scatter::quality::*;
use crate::third_party::avian3d::CollisionLayer;
use crate::{RenderLayer, RenderLayers};

use avian3d::prelude::*;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy_eidolon::prelude::*;
use bevy_feronia::prelude::*;

#[derive(Component)]
#[component(on_add = Self::on_add)]
#[require(
    Name::new("Rock Layer"),
    ScatterLayerType::<StandardMaterial>,
    InstanceRotationYaw,
    InstanceScale,
    InstanceScaleRange{
		min: 8.,
	    max: 16.
	},
    InstanceJitter,
    Avoidance(1.6),
    DistributionDensity(25.),
    ScatterPhysicsBody
)]
pub(crate) struct RockLayer;

impl RockLayer {
	fn on_add(mut world: DeferredWorld, ctx: HookContext) {
		let EnvironmentAssets {
			rocks,
			rock_density_map,
			..
		} = world
			.get_resource::<EnvironmentAssets>()
			.cloned()
			.expect("Assets should be added!");

		let settings = world.resource::<QualitySetting>().clone();
		let density_settings = RockDensitySetting::from(settings);
		let visibility_settings = RockVisibilityRangeQuality::from(settings);
		let collider_hierarchy =
			ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
				.with_default_layers(CollisionLayers::new(
					CollisionLayer::Default,
					LayerMask::ALL,
				));

		let mut cmd = world.commands();

		cmd.entity(ctx.entity).insert((
			DistributionPattern(rock_density_map),
			DistributionDensity::from(density_settings),
			LodConfig::from(visibility_settings),
		));

		cmd.spawn((ChildOf(ctx.entity), SceneRoot(rocks), collider_hierarchy))
			.observe(disable_shadow_casting_on_instance_ready);
	}
}

#[derive(Component)]
#[component(on_add = Self::on_add)]
#[require(
    Name::new("Mushroom Layer"),
    ScatterLayerType::<ExtendedWindAffectedMaterial>,
    InstanceRotationYaw,
	ScatterPhysicsBody,
    InstanceScale,
	InstanceScaleRange {
       min: 4.,
	   max: 32.
	},
    InstanceJitter,
	Strength(0.2),
	MicroStrength(0.1),
	SCurveStrength(0.1),
	BopStrength(0.2),
    Avoidance(0.02),
	WindAffected,
	SubsurfaceScattering,
)]
pub struct MushroomLayer;

impl MushroomLayer {
	fn on_add(mut world: DeferredWorld, ctx: HookContext) {
		let EnvironmentAssets {
			mushroom,
			mushroom_density_map,
			..
		} = world
			.get_resource::<EnvironmentAssets>()
			.cloned()
			.expect("Assets should be added!");

		let settings = world.resource::<QualitySetting>().clone();
		let density_settings = MushroomDensitySetting::from(settings);
		let visibility_settings = MushroomVisibilityRangeQuality::from(settings);
		let collider_hierarchy =
			ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
				.with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL));

		let mut cmd = world.commands();

		cmd.entity(ctx.entity).insert((
			DistributionPattern(mushroom_density_map),
			DistributionDensity::from(density_settings),
			LodConfig::from(visibility_settings),
		));

		cmd.spawn_batch([(
			ChildOf(ctx.entity),
			SceneRoot(mushroom),
			RigidBody::Static,
			collider_hierarchy,
		)]);
	}
}

#[derive(Component)]
#[component(on_add = Self::on_add)]
#[require(
    Name::new("Grass Layer"),
    ScatterLayerType::<InstancedWindAffectedMaterial>,

    // Scatter options

    InstanceJitter,
    InstanceScale,
    ScatterChunked,
    ScaleDensity,

    // Material options
	WindAffected,
    CurveNormals,
    AnalyticalNormals,
    InstanceRotationYaw,
    StandardPbr,
    SubsurfaceScattering,
	InstanceColor::new(Srgba::hex("#1f3114").unwrap()),
	InstanceColorGradient {
		end: 0.7,
		start: 0.2,
		..InstanceColorGradient::new(
			Srgba::hex("#3e6328").unwrap(),
			Srgba::hex("#0f190a").unwrap()
		)
	},
    StaticBend,
    AmbientOcclusion,
	MicroStrength(1.2),
	GpuCullCompute,
	RenderLayers::from(RenderLayer::GRASS),
)]
pub(crate) struct GrassLayer;

impl GrassLayer {
	fn on_add(mut world: DeferredWorld, ctx: HookContext) {
		let EnvironmentAssets {
			grass,
			grass_med,
			grass_low,
			grass_density_map,
			..
		} = world.resource::<EnvironmentAssets>().clone();

		let settings = world.resource::<QualitySetting>().clone();
		let density_settings = GrassDensitySetting::from(settings);
		let visibility_settings = GrassVisibilityRangeQuality::from(settings);
		let collider_hierarchy =
			ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh);

		let mut cmd = world.commands();

		cmd.entity(ctx.entity).insert((
			DistributionPattern(grass_density_map),
			DistributionDensity::from(density_settings),
			LodConfig::from(visibility_settings),
		));

		cmd.spawn_batch([
			(
				SceneRoot(grass),
				ChildOf(ctx.entity),
				LevelOfDetail(0),
				collider_hierarchy.clone(),
			),
			(
				SceneRoot(grass_med),
				ChildOf(ctx.entity),
				LevelOfDetail(1),
				collider_hierarchy.clone(),
			),
			(
				SceneRoot(grass_low),
				ChildOf(ctx.entity),
				LevelOfDetail(2),
				collider_hierarchy,
			),
		]);
	}
}
