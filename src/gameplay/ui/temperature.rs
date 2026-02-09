use bevy::{color::palettes::css, prelude::*, ui_widgets::observe};
use bevy_notify::prelude::*;

use crate::gameplay::core::Temperature;

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
		BIT_CHILLY.interpolate_stable(&ITS_FINE, temp / 20.)
	} else if temp < 37. {
		ITS_FINE.interpolate_stable(&UNCOMFORTABLE, temp / 17.)
	} else if temp < 50. {
		UNCOMFORTABLE.interpolate_stable(&THIRD_DEGREE_BURN, temp / 13.)
	} else if temp < 500. {
		THIRD_DEGREE_BURN.interpolate_stable(&OH_ITS_BLUE_NOW, temp / 450.)
	} else {
		OH_ITS_BLUE_NOW.interpolate_stable(&IVE_SUMMONED_THE_SUN, temp / f32::MAX)
	}
}

#[derive(Component)]
pub struct TemperatureBar;

pub fn temperature_bar(player: Entity) -> impl Bundle {
	(
		TemperatureBar,
		Node {
			width: px(25),
			..Default::default()
		},
		BackgroundColor(ITS_FINE.into()),
		Monitor(player),
		NotifyChanged::<Temperature>::default(),
		observe(
			|mutation: On<Mutation<Temperature>>,
			 mut nodes: Query<(&mut Node, &mut BackgroundColor)>,
			 temperature: Query<&Temperature>|
			 -> Result<(), BevyError> {
				let (mut bar, mut background) = nodes.get_mut(mutation.entity)?;
				let current_temperature = temperature.get(mutation.mutated)?;

				bar.height = percent(**current_temperature);
				let new_background = temp_to_color(**current_temperature);

				dbg!(new_background);
				background.0 = temp_to_color(**current_temperature).into();

				Ok(())
			},
		),
	)
}
