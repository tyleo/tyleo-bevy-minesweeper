use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum BoardPositionOption {
    /// Centered board
    Centered { offset: Vec3 },

    /// Custom position
    Custom(Vec3),
}

impl Default for BoardPositionOption {
    fn default() -> Self {
        Self::Centered { offset: default() }
    }
}
