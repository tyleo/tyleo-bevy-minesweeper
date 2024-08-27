use crate::resources::*;
use bevy::{math::U16Vec2, prelude::*};
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

impl BoardOptions {
    pub fn compute_tile_size(&self, window_size: &Vec2, tile_map_size: U16Vec2) -> f32 {
        match self.tile_size {
            TileSizeOption::Fixed(v) => v,
            TileSizeOption::Adaptive { min, max } => {
                Self::compute_adaptive_tile_size(window_size, &(min, max), tile_map_size)
            }
        }
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn compute_adaptive_tile_size(
        window_size: &Vec2,
        (min, max): &(f32, f32),
        tile_map_size: U16Vec2,
    ) -> f32 {
        let width = tile_map_size.x as f32;
        let height = tile_map_size.y as f32;

        let max_width = window_size.x / width;
        let max_height = window_size.y / height;

        max_width.min(max_height).clamp(*min, *max)
    }

    pub fn optional_resource_or_default(v: Option<Res<BoardOptions>>) -> BoardOptions {
        match v {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        }
    }
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
