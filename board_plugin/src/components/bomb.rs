use bevy::prelude::*;

/// A bomb on the board
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Component, Reflect)]
pub struct Bomb;
