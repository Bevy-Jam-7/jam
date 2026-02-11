use bevy::prelude::*;
use bevy_bae::prelude::*;

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
) -> OperatorStatus {
	OperatorStatus::Ongoing
}
