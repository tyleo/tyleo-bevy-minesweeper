use crate::{components::*, resources::*, util::*};
use bevy::{log, prelude::*, utils::HashMap};

#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
    pub marked_tiles: Vec<Coordinates>,
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
        if self.marked_tiles.contains(coordinates) {
            // Return `None` if the tile is marked to prevent marked tiles from being uncovered
            None
        } else {
            self.covered_tiles.get(coordinates)
        }
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        if self.marked_tiles.contains(coordinates) {
            self.unmark_tile(coordinates)?;
        }

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

    /// Removes the `coords` from `marked_tiles`
    fn unmark_tile(&mut self, coordinates: &Coordinates) -> Option<Coordinates> {
        let tile_index = match self
            .marked_tiles
            .iter()
            .position(|current_coordinates| current_coordinates == coordinates)
        {
            None => {
                log::error!("Failed to unmark tile at {}", coordinates);
                return None;
            }
            Some(tile_index) => tile_index,
        };
        Some(self.marked_tiles.remove(tile_index))
    }

    /// Is the board complete
    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }

    /// We try to mark or unmark a tile, returning the entity and if the tile is marked
    pub fn try_toggle_mark(&mut self, coordinates: &Coordinates) -> Option<(&Entity, bool)> {
        let mark = if self.marked_tiles.contains(coordinates) {
            self.unmark_tile(coordinates)?;
            false
        } else {
            self.marked_tiles.push(*coordinates);
            true
        };

        let covered_tile_entity = self.covered_tiles.get(coordinates)?;
        Some((covered_tile_entity, mark))
    }
}
