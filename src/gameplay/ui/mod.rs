pub mod temperature;

use bevy::prelude::*;

use crate::{gameplay::player::Player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
	app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);
}

pub(crate) fn spawn_hud(mut commands: Commands, player: Single<Entity, With<Player>>) {
	commands.spawn(hud(*player));
}

fn hud(player: Entity) -> impl Bundle {
	(
		Node {
			width: percent(100),
			height: percent(100),
			padding: percent(2).all(),
			flex_direction: FlexDirection::ColumnReverse,
			..Default::default()
		},
		children![temperature::temperature_bar(player)],
	)
}
