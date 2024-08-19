use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum TileSizeOption {
    /// Fixed tile size
    Fixed(f32),

    /// Tile size changes with the size of the window
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSizeOption {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}
