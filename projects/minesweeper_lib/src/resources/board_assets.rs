use crate::resources::*;
use bevy::prelude::*;

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub pending_tile_material: SpriteMaterial,
    pub bomb_number_font: Handle<Font>,
    pub bomb_number_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub bomb_material: SpriteMaterial,
}

impl BoardAssets {
    /// Safely retrieves the color matching a bomb counter
    pub fn bomb_number_color(&self, number: u8) -> Color {
        let counter = number.saturating_sub(1) as usize;
        match self.bomb_number_colors.get(counter) {
            Some(c) => *c,
            None => match self.bomb_number_colors.last() {
                None => Color::WHITE,
                Some(c) => *c,
            },
        }
    }
}
