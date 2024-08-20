use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Resource)]
pub struct BoardColors {
    pub background_color: Color,
    pub padding_color: Color,
    pub unknown_tile_color: Color,
    pub revealed_tile_color: Color,
    pub highlighted_tile_color: Color,
    pub flag_color: Color,
    pub bomb_color: Color,
    pub number_colors: Vec<Color>,
}

impl Default for BoardColors {
    fn default() -> Self {
        Self {
            background_color: Color::srgb_u8(19, 20, 22),
            padding_color: Color::srgb_u8(19, 20, 22),
            unknown_tile_color: Color::srgb_u8(59, 63, 68),
            revealed_tile_color: Color::srgb_u8(26, 27, 30),
            highlighted_tile_color: Color::srgb_u8(71, 75, 82),
            flag_color: Color::srgb_u8(27, 167, 223),
            bomb_color: Color::srgb_u8(241, 91, 80),
            number_colors: vec![
                Color::WHITE,
                Color::srgb_u8(64, 182, 73),  // green
                Color::srgb_u8(228, 208, 32), // yellow
                Color::srgb_u8(250, 131, 20), // orange
                Color::srgb_u8(178, 21, 214), // purple
            ],
        }
    }
}
