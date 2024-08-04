use bevy::prelude::*;

/// Identifies uncovered tiles
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Component, Reflect)]
pub struct Uncover;
