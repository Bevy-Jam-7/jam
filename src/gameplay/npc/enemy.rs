use bevy::prelude::*;
use bevy_bae::prelude::*;
use bevy_landmass::Archipelago3d;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
	app.add_systems(FixedUpdate, update_sensors.before(BaeSystems::ExecutePlan));
}

fn update_sensors() {}

fn enemy_htn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
	commands.spawn((
		Plan::new(),
		Select,
		tasks![(
			conditions![Condition::eq("close_to_cursor", true)],
			Operator::new(walk_randomly),
		),],
	));
}

fn walk_randomly(
	In(input): In<OperatorInput>,
	mut transforms: Query<&mut Transform>,
	archipelago: Single<&Archipelago3d>,
) -> OperatorStatus {
	let offset = Circle::new(3.0);
	archipelago.sample_point(point, point_sample_distance);
	OperatorStatus::Ongoing
}

#[point_class(base(Transform, Visibility))]
struct MyCoolEmitter {
	some_property: String,
	some_other_property: i32,
}
impl Default for MyCoolEmitter {
	fn default() -> Self {
		Self {
			some_property: "default".to_string(),
			some_other_property: 0,
		}
	}
}
