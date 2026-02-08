use bevy::prelude::*;
use bevy_seedling::firewheel::nodes::svf::SvfNode;
use bevy_seedling::prelude::*;

use crate::menus::Menu;
use animation::{AnimateCutoff, play_at};

pub(crate) mod animation;
pub(crate) mod perceptual;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_audio)
        .register_node::<SvfNode<2>>()
        .register_type::<SvfNode<2>>()
        .add_systems(Update, manage_filter_enabled)
        .add_systems(OnExit(Menu::None), enable_music_filter)
        .add_systems(OnEnter(Menu::None), disable_music_filter);
}

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct SpatialPool;

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct SfxPool;

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct MusicPool;

#[derive(Component)]
pub(crate) struct MusicFilter;

// fn disable_music_filter(mut filter: Single<&mut SvfNode, With<MusicFilter>>) {
//     filter.enabled = false;
// }

/// Set somewhere below 0 dB so that the user can turn the volume up if they want to.
pub(crate) const DEFAULT_MAIN_VOLUME: Volume = Volume::Linear(0.5);

fn initialize_audio(server: Res<AssetServer>, time: Res<Time<Audio>>, mut commands: Commands) {
    // Tuned by ear
    const DEFAULT_POOL_VOLUME: Volume = Volume::Linear(1.6);

    // Buses
    commands
        .spawn((
            MainBus,
            VolumeNode {
                volume: DEFAULT_MAIN_VOLUME,
                ..Default::default()
            },
            Name::new("Main Bus"),
        ))
        .chain_node(LimiterNode::new(0.003, 0.15))
        .connect(AudioGraphOutput);

    commands.spawn((
        SoundEffectsBus,
        VolumeNode::default(),
        Name::new("Sound Effects Bus"),
    ));

    commands
        .spawn((
            Name::new("Music audio sampler pool"),
            SamplerPool(MusicPool),
            VolumeNode { ..default() },
        ))
        // we'll add a cute filter for menus
        .chain_node((
            SvfNode::<2> {
                filter_type: firewheel::nodes::svf::SvfType::LowpassX2,
                q_factor: 1.5,
                cutoff_hz: 800.0,
                enabled: true,
                ..default()
            },
            MusicFilter,
        ));

    commands
        .spawn((
            Name::new("SFX audio sampler pool"),
            SamplerPool(SpatialPool),
            sample_effects![(SpatialBasicNode::default(), SpatialScale(Vec3::splat(2.0)))],
            VolumeNode {
                volume: DEFAULT_POOL_VOLUME,
                ..default()
            },
        ))
        .connect(SoundEffectsBus);

    commands
        .spawn((
            Name::new("UI SFX audio sampler pool"),
            SamplerPool(SfxPool),
            VolumeNode {
                volume: DEFAULT_POOL_VOLUME,
                ..default()
            },
        ))
        .connect(SoundEffectsBus);

    play_penis_music(&time, &server, commands.reborrow());
}

fn play_penis_music(time: &Time<Audio>, server: &AssetServer, mut commands: Commands) {
    let start_time = 1.0 + 4.0 / 3.0;
    commands.spawn((
        play_at(
            SamplePlayer::new(server.load("audio/music/penis-music/intro.wav"))
                .with_volume(Volume::Decibels(9.0)),
            time,
            1.05, // looks like we need to investigate timing issues
        ),
        MusicPool,
    ));

    commands.spawn((
        play_at(
            SamplePlayer::new(server.load("audio/music/penis-music/dnb.wav"))
                .looping()
                .with_volume(Volume::Decibels(9.0)),
            time,
            start_time,
        ),
        MusicPool,
    ));
    commands.spawn((
        play_at(
            SamplePlayer::new(server.load("audio/music/penis-music/voices.wav"))
                .looping()
                .with_volume(Volume::Decibels(9.0)),
            time,
            start_time,
        ),
        MusicPool,
    ));
    commands.spawn((
        play_at(
            SamplePlayer::new(server.load("audio/music/penis-music/lead.wav"))
                .looping()
                .with_volume(Volume::Decibels(9.0)),
            time,
            start_time,
        ),
        MusicPool,
    ));
}

// Sweep the filter down when entering a menu.
fn enable_music_filter(
    filter: Single<(&SvfNode, &mut AudioEvents), With<MusicFilter>>,
    time: Res<Time<Audio>>,
) {
    let (node, mut events) = filter.into_inner();
    node.animate_cutoff(800.0, 0.3, &time, &mut events);
}

// Sweep the filter back up when exiting a menu.
fn disable_music_filter(
    filter: Single<(&SvfNode, &mut AudioEvents), With<MusicFilter>>,
    time: Res<Time<Audio>>,
) {
    let (node, mut events) = filter.into_inner();
    node.animate_cutoff(20_000.0, 0.6, &time, &mut events);
}

// I want to make sure the filter is always disabled when above 20kHz.
//
// This is a bit more robust than scheduling events, since this can't be dropped.
fn manage_filter_enabled(filters: Query<&mut SvfNode, Changed<SvfNode>>) {
    for mut filter in filters {
        if filter.cutoff_hz >= 20_000.0 && filter.enabled {
            filter.enabled = false;
        } else if filter.cutoff_hz < 20_000.0 && !filter.enabled {
            filter.enabled = true;
        }
    }
}
