use crate::{components::*, resources::*, util::*};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
}

impl Board {
    /// Translates a mouse psition to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Convert window space to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }

        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: self.tile_map.height() - ((coordinates.y / self.tile_size) as u16) - 1,
        })
    }

    /// Retrieves a covered tile entity
    pub fn tile_to_uncover(&self, coordinates: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coordinates)
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coordinates)
    }

    /// We retrieve the adjacent covered tile entities of some `coordinates`
    pub fn adjacent_covered_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .iter_neighbors(coordinates)
            .filter_map(|neighbor_coordinates| self.covered_tiles.get(&neighbor_coordinates))
            .copied()
            .collect()
    }
}
