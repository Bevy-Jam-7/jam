/// Can be split up and renamed later.
use bevy::prelude::*;
use std::time::Duration;

/// Temperature of an entity
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Clone, Debug, Component)]
pub struct Temperature(pub f32);

impl Default for Temperature {
    fn default() -> Self {
        Self(37.0)
    }
}

/// Base damage of a unit/entity (could be modified by temperature/fever).
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Clone, Debug, Component)]
pub struct Damage(pub f32);

impl Default for Damage {
    fn default() -> Self {
        Self(10.)
    }
}

/// Global temperature of the world
#[derive(Resource, Debug, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Clone, Debug, Resource)]
pub struct GlobalTemperature(pub f32);

impl Default for GlobalTemperature {
    fn default() -> Self {
        Self(20.)
    }
}

/// Multiplier for the global temperature can be placed in the world.
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Clone, Debug, Component)]
pub struct EnvironmentTemperature(pub f32);

impl Default for EnvironmentTemperature {
    fn default() -> Self {
        Self(40.)
    }
}

/// Base health of a unit/entity (could be modified by temperature).
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Clone, Debug, Component)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Self(100.)
    }
}

/// Timer used to track the `fever over time` effect. Allows for slowing down or speeding up the effect.
#[derive(Component, Debug, Deref, DerefMut, Clone, Reflect)]
#[reflect(Clone, Debug, Component)]
pub struct FeverTimer(pub Timer);

impl Default for FeverTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), TimerMode::Repeating))
    }
}
