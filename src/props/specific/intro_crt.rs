//! Handles the animated expressions for the intro CRT

use crate::{
	asset_tracking::LoadResource as _, gameplay::dialogue_view::typewriter::Typewriter,
	props::generic::Crt, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_yarnspinner::prelude::DialogueRunner;
use std::collections::VecDeque;

pub(super) fn plugin(app: &mut App) {
	app.init_resource::<CrtModel>()
		.init_resource::<CrtScreenTextures>()
		.init_resource::<CrtEmoteCommandBuffer>()
		.add_systems(
			Update,
			(
				poll_crt_model_load.run_if(not(resource_exists::<CrtScreenMaterial>)),
				clear_intro_crt_emote,
				update_intro_crt_emotes.run_if(|b: Option<Res<CrtEmoteCommandBuffer>>| {
					b.is_some_and(|b| !b.is_empty())
				}),
			),
		);
	#[cfg(feature = "dev")]
	{
		app.load_asset::<Image>("models/office/crt/boot.png")
			.load_asset::<Image>("models/office/crt/smile.png")
			.load_asset::<Image>("models/office/crt/smile2.png")
			.load_asset::<Image>("models/office/crt/stonks.png")
			.load_asset::<Image>("models/office/crt/sideways.png")
			.load_asset::<Image>("models/office/crt/point.png")
			.load_asset::<Image>("models/office/crt/glitch.png")
			.load_asset::<Image>("models/office/crt/shocked.png")
			.load_asset::<Image>("models/office/crt/nod.png")
			.load_asset::<Image>("models/office/crt/shake.png")
			.load_asset::<Image>("models/office/crt/blank.png")
			.load_asset::<Image>("models/office/crt/x.png")
			.load_asset::<Image>("models/office/crt/annoyed.png")
			.load_asset::<Image>("models/office/crt/away.png")
			.load_asset::<Image>("models/office/crt/upside.png")
			.load_asset::<Image>("models/office/crt/leftside.png")
			.load_asset::<Image>("models/office/crt/rightside.png");
	}
	#[cfg(feature = "release")]
	{
		app.load_asset::<Image>("models/office/crt/boot.ktx2")
			.load_asset::<Image>("models/office/crt/smile.ktx2")
			.load_asset::<Image>("models/office/crt/smile2.ktx2")
			.load_asset::<Image>("models/office/crt/stonks.ktx2")
			.load_asset::<Image>("models/office/crt/sideways.ktx2")
			.load_asset::<Image>("models/office/crt/point.ktx2")
			.load_asset::<Image>("models/office/crt/glitch.ktx2")
			.load_asset::<Image>("models/office/crt/shocked.ktx2")
			.load_asset::<Image>("models/office/crt/nod.ktx2")
			.load_asset::<Image>("models/office/crt/shake.ktx2")
			.load_asset::<Image>("models/office/crt/blank.ktx2")
			.load_asset::<Image>("models/office/crt/x.ktx2")
			.load_asset::<Image>("models/office/crt/annoyed.ktx2")
			.load_asset::<Image>("models/office/crt/away.ktx2")
			.load_asset::<Image>("models/office/crt/upside.ktx2")
			.load_asset::<Image>("models/office/crt/leftside.ktx2")
			.load_asset::<Image>("models/office/crt/rightside.ktx2");
	}
}

/// Call this system from yarnspinner to change the emote
pub(crate) fn set_intro_crt_emote(
	In((emote_name, delay_graphemes)): In<(String, Option<usize>)>,
	mut commands: ResMut<CrtEmoteCommandBuffer>,
) {
	let delay_graphemes = delay_graphemes.unwrap_or_default();
	commands.push_back(EmoteCommand {
		emote_name,
		delay_graphemes,
	});
}

/// Stores deferred commands to change emotes
#[derive(Resource, Deref, DerefMut, Default)]
pub(crate) struct CrtEmoteCommandBuffer(VecDeque<EmoteCommand>);

pub(crate) struct EmoteCommand {
	emote_name: String,
	delay_graphemes: usize,
}

/// Repository for the images that can be used as the intro CRT emote
#[derive(Resource, Deref)]
struct CrtScreenTextures(HashMap<String, Handle<Image>>);

impl FromWorld for CrtScreenTextures {
	fn from_world(world: &mut World) -> Self {
		let assets = world.resource::<AssetServer>();

		#[cfg(feature = "dev")]
		{
			Self(HashMap::from_iter([
				("boot".into(), assets.load("models/office/crt/boot.png")),
				("smile".into(), assets.load("models/office/crt/smile.png")),
				("smile2".into(), assets.load("models/office/crt/smile2.png")),
				("stonks".into(), assets.load("models/office/crt/stonks.png")),
				(
					"sideways".into(),
					assets.load("models/office/crt/sideways.png"),
				),
				("point".into(), assets.load("models/office/crt/point.png")),
				("glitch".into(), assets.load("models/office/crt/glitch.png")),
				(
					"shocked".into(),
					assets.load("models/office/crt/shocked.png"),
				),
				("nod".into(), assets.load("models/office/crt/nod.png")),
				("shake".into(), assets.load("models/office/crt/shake.png")),
				("blank".into(), assets.load("models/office/crt/blank.png")),
				("x".into(), assets.load("models/office/crt/x.png")),
				(
					"annoyed".into(),
					assets.load("models/office/crt/annoyed.png"),
				),
				("away".into(), assets.load("models/office/crt/away.png")),
				("upside".into(), assets.load("models/office/crt/upside.png")),
				(
					"leftside".into(),
					assets.load("models/office/crt/leftside.png"),
				),
				(
					"rightside".into(),
					assets.load("models/office/crt/rightside.png"),
				),
			]))
		}
		#[cfg(feature = "release")]
		{
			Self(HashMap::from_iter([
				("boot".into(), assets.load("models/office/crt/boot.ktx2")),
				("smile".into(), assets.load("models/office/crt/smile.ktx2")),
				(
					"smile2".into(),
					assets.load("models/office/crt/smile2.ktx2"),
				),
				(
					"stonks".into(),
					assets.load("models/office/crt/stonks.ktx2"),
				),
				(
					"sideways".into(),
					assets.load("models/office/crt/sideways.ktx2"),
				),
				("point".into(), assets.load("models/office/crt/point.ktx2")),
				(
					"glitch".into(),
					assets.load("models/office/crt/glitch.ktx2"),
				),
				(
					"shocked".into(),
					assets.load("models/office/crt/shocked.ktx2"),
				),
				("nod".into(), assets.load("models/office/crt/nod.ktx2")),
				("shake".into(), assets.load("models/office/crt/shake.ktx2")),
				("blank".into(), assets.load("models/office/crt/blank.ktx2")),
				("x".into(), assets.load("models/office/crt/x.ktx2")),
				(
					"annoyed".into(),
					assets.load("models/office/crt/annoyed.ktx2"),
				),
				("away".into(), assets.load("models/office/crt/away.ktx2")),
				(
					"upside".into(),
					assets.load("models/office/crt/upside.ktx2"),
				),
				(
					"leftside".into(),
					assets.load("models/office/crt/leftside.ktx2"),
				),
				(
					"rightside".into(),
					assets.load("models/office/crt/rightside.ktx2"),
				),
			]))
		}
	}
}

#[derive(Resource, Deref)]
struct CrtScreenMaterial(Handle<StandardMaterial>);

#[derive(Resource, Deref)]
struct CrtModel(Handle<Gltf>);

impl FromWorld for CrtModel {
	fn from_world(world: &mut World) -> Self {
		Self(world.load_asset(Crt::model_path()))
	}
}

fn poll_crt_model_load(handle: Res<CrtModel>, models: Res<Assets<Gltf>>, mut commands: Commands) {
	if let Some(model) = models.get(&**handle) {
		if let Some(material) = model.named_materials.get("glass") {
			commands.insert_resource(CrtScreenMaterial(material.clone()));
			commands.remove_resource::<CrtModel>();
		} else {
			warn!("Could not get glass material from monitor model");
		}
	}
}

fn update_intro_crt_emotes(
	handle: Res<CrtScreenMaterial>,
	textures: Res<CrtScreenTextures>,
	typewriter: Res<Typewriter>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut commands: ResMut<CrtEmoteCommandBuffer>,
) {
	if let Some(material) = materials.get_mut(&**handle) {
		while let Some(first) = commands.front()
			&& typewriter.elapsed_graphemes() >= first.delay_graphemes
		{
			let texture = textures.get(&first.emote_name);
			// Paste the texture or clear it
			material.emissive_texture = texture.cloned();
			if texture.is_some() {
				material.emissive = Color::WHITE.to_linear() * 20.0;
			} else {
				material.emissive = Color::BLACK.into();
			}
			commands.pop_front();
		}
	} else {
		warn!("Could not get glass material from asset repository");
	}
}

/// Clears the emote when dialog stops
/// so the face does not show up on every screen
/// in a 5 mile radius
fn clear_intro_crt_emote(
	dialog: Single<&DialogueRunner>,
	handle: Res<CrtScreenMaterial>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut commands: ResMut<CrtEmoteCommandBuffer>,
) {
	if !dialog.is_running()
		&& let Some(material) = materials.get_mut(&**handle)
	{
		material.emissive = Color::BLACK.into();
		material.emissive_texture = None;
		commands.clear();
	}
}
