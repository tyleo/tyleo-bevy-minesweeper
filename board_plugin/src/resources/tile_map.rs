use crate::{components::*, resources::*};
use rand::{thread_rng, Rng};

// A grid of tiles play area.
#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    // Stores tiles in left-to-right then bottom-to-top layout
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    /// Creates a new `TileMap` with the specified bomb count.
    pub fn new_with_bombs(width: u16, height: u16, bomb_count: u16) -> Self {
        let mut result = Self::empty(width, height);
        result.bomb_count = bomb_count;

        // Place bombs
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..result.width) as usize,
                rng.gen_range(0..result.height) as usize,
            );

            if let Tile::Empty = result.map[y][x] {
                result.map[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        // Place bomb neighbors
        for y in 0..result.height {
            for x in 0..result.width {
                let current = Coordinates { x, y };
                if result.is_bomb_at(current) {
                    continue;
                }

                let bomb_count = result.bomb_count_at(current);
                if bomb_count == 0 {
                    continue;
                }

                let tile = &mut result.map[y as usize][x as usize];
                *tile = Tile::BombNeighbor(bomb_count);
            }
        }

        result
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn get_map(&self) -> &Vec<Vec<Tile>> {
        &self.map
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs: \n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);

        for line in self.map.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }

        format!("{}{}", buffer, line)
    }

    /// Returns an slice of the offset coordinates which can be subtracted from a coordinate to find
    /// its neighbor coordinates in the following order:
    /// 1. Bottom Left
    /// 2. Bottom Center
    /// 3. Bottom Right
    /// 4. Middle Left
    /// 5. Middle Right
    /// 6. Top Left
    /// 7. Top Center
    /// 8. Top Right
    const OFFSET_COORDINATES_OF_NEIGHBORS: [(i8, i8); 8] = [
        // Bottom
        (-1, -1),
        (0, -1),
        (1, -1),
        // Middle
        (-1, 0),
        (1, 0),
        // Top
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    /// Returns the neighbor coordinates of the coordinates in the following order:
    /// 1. Bottom Left
    /// 2. Bottom Center
    /// 3. Bottom Right
    /// 4. Middle Left
    /// 5. Middle Right
    /// 6. Top Left
    /// 7. Top Center
    /// 8. Top Right
    pub fn iter_neighbors(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        Self::OFFSET_COORDINATES_OF_NEIGHBORS
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    /// Returns true if there as a bomb at the coordinates; otherwise false
    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        }

        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    // Returns the bomb count at the specified coordinates
    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        let res = self
            .iter_neighbors(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }
}
