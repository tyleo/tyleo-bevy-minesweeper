use bevy::prelude::*;

/// Identifies highlighted tiles
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Component, Reflect)]
pub struct Cover;
