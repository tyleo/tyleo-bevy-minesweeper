mod type_registry;

pub mod components;
pub mod ext;
pub mod resources;
pub mod util;

use components::Coordinates;
pub use type_registry::*;

use crate::resources::{BoardOptions, BoardPositionOption, TileSizeOption};
use bevy::log;
use bevy::prelude::*;
use resources::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window>,
    ) {
        // Create the tile map
        let tile_map = TileMap::new_with_bombs(20, 20, 40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let window = windows.get_single().unwrap();

        let board_options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_size = match board_options.tile_size {
            TileSizeOption::Fixed(v) => v,
            TileSizeOption::Adaptive { min, max } => Self::compute_adaptive_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        let board_position = match board_options.position {
            BoardPositionOption::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPositionOption::Custom(p) => p,
        };

        commands
            .spawn_empty()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .insert(InheritedVisibility::default())
            .with_children(|parent| {
                // Spawn background
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                // Spawn tiles
                for (y, tile_row) in tile_map.map().iter().enumerate() {
                    for (x, _) in tile_row.iter().enumerate() {
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::linear_rgb(0.1, 0.1, 0.1),
                                    custom_size: Some(Vec2::splat(
                                        tile_size - board_options.tile_padding,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn compute_adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let window_width = window.resolution.physical_width() as f32;
        let window_height = window.resolution.physical_height() as f32;

        let max_width = window_width / width as f32;
        let max_heigth = window_height / height as f32;

        max_width.min(max_heigth).clamp(min, max)
    }
}
