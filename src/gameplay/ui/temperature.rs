use bevy::{prelude::*, ui_widgets::observe};
use bevy_notify::prelude::*;

use crate::{
	gameplay::core::Temperature,
	theme::widget::{StatBarDirection, stat_bar},
};

const MARGRET_THATCHER_HAS_ESCAPED_HER_GRAVE: Srgba = Srgba::rgb_u8(16, 8, 30);
const BIT_CHILLY: Srgba = Srgba::rgb_u8(100, 147, 186);
const ITS_FINE: Srgba = Srgba::rgb_u8(194, 225, 249);
const UNCOMFORTABLE: Srgba = Srgba::rgb_u8(242, 147, 75);
const THIRD_DEGREE_BURN: Srgba = Srgba::rgb_u8(234, 29, 2);
const OH_ITS_BLUE_NOW: Srgba = Srgba::rgb_u8(0, 108, 240);
const IVE_SUMMONED_THE_SUN: Srgba = Srgba::rgb_u8(255, 255, 255);

fn temp_to_color(temp: f32) -> Srgba {
	if temp < 0. {
		MARGRET_THATCHER_HAS_ESCAPED_HER_GRAVE.interpolate_stable(&BIT_CHILLY, temp / f32::MIN)
	} else if temp < 20. {
		BIT_CHILLY.interpolate_stable(&ITS_FINE, temp)
	} else if temp < 37. {
		ITS_FINE.interpolate_stable(&UNCOMFORTABLE, (temp - 20.) / 17.)
	} else if temp < 50. {
		UNCOMFORTABLE.interpolate_stable(&THIRD_DEGREE_BURN, (temp - 37.) / 13.)
	} else if temp < 500. {
		THIRD_DEGREE_BURN.interpolate_stable(&OH_ITS_BLUE_NOW, (temp - 50.) / 450.)
	} else {
		OH_ITS_BLUE_NOW.interpolate_stable(&IVE_SUMMONED_THE_SUN, (temp - 500.0) / f32::MAX)
	}
}

#[derive(Component)]
pub struct TemperatureBar;

pub fn temperature_bar(player: Entity) -> impl Bundle {
	(
		TemperatureBar,
		BackgroundColor(ITS_FINE.into()),
		stat_bar::<Temperature>(player, StatBarDirection::Vertical),
		observe(update_color),
	)
}

fn update_color(
	mutation: On<Mutation<Temperature>>,
	mut background_color: Query<&mut BackgroundColor>,
	temperature: Query<&Temperature>,
) -> Result<(), BevyError> {
	let mut background_color = background_color.get_mut(mutation.entity)?;
	let temperature = temperature.get(mutation.mutated)?;

	background_color.0 = temp_to_color(**temperature).into();

	Ok(())
}
