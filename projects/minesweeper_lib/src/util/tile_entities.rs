use bevy::prelude::*;

/// Collects all of the entities related to a tile
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Reflect)]
pub struct TileEntities {
    pub root: Entity,
    pub cover: Entity,
    pub kind: Option<Entity>,
}
