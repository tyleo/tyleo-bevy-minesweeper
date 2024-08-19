use bevy::prelude::*;

/// The neighbor of a bomb
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Component, Reflect)]
pub struct BombNeighbor {
    /// The number of neighbor bombs
    pub number: u8,
}
