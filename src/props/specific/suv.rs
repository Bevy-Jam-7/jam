use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::{
	asset_tracking::LoadResource as _,
	props::effects::disable_shadow_casting_on_instance_ready,
	third_party::bevy_trenchbroom::{GetTrenchbroomModelPath as _, LoadTrenchbroomModel as _},
};

pub(super) fn plugin(app: &mut App) {
	app.add_observer(on_add_suv)
		.load_asset::<Gltf>(Suv::model_path());
}

#[point_class(base(Transform, Visibility), model("models/suv/suv.gltf"))]
pub(crate) struct Suv;

fn on_add_suv(add: On<Add, Suv>, mut commands: Commands, asset_server: Res<AssetServer>) {
	let model = asset_server.load_trenchbroom_model::<Suv>();
	commands
		.entity(add.entity)
		.insert(SceneRoot(model))
		.observe(disable_shadow_casting_on_instance_ready);
}
