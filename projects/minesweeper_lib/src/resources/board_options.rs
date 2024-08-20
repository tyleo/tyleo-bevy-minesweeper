use crate::resources::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Board generation options which must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Resource)]
#[reflect(Resource)]
pub struct BoardOptions {
    /// Tile map size
    pub map_size: (u16, u16),

    /// The number of bombs
    pub bomb_count: u16,

    /// Board world position
    pub position: BoardPositionOption,

    /// Tile world size
    pub tile_size: TileSizeOption,

    /// Padding between tiles
    pub tile_padding: f32,

    /// Does the board generate a safe place to start
    pub safe_start: bool,

    /// The colors of the board
    pub colors: BoardColors,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: default(),
            tile_size: default(),
            tile_padding: 0.,
            safe_start: false,
            colors: default(),
        }
    }
}
