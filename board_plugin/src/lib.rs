mod bounds2;
mod type_registry;

pub mod components;
pub mod ext;
pub mod resources;
pub mod systems;
pub mod util;

pub use bounds2::*;
pub use type_registry::*;

use crate::{components::*, resources::*, systems::*};
use bevy::log;
use bevy::prelude::*;
use components::Coordinates;
use resources::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        app.add_systems(FixedUpdate, input_handling);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window>,
        asset_server: Res<AssetServer>,
    ) {
        // Load assets
        let font = asset_server.load("fonts/arial-rounded-mt-regular.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");

        // Create the tile map
        let tile_map = TileMap::new_with_bombs(20, 20, 40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let window = windows.get_single().unwrap();

        let board_options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_padding = board_options.tile_padding;
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

        // Create entities
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
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    tile_padding,
                    Color::linear_rgb(0.1, 0.1, 0.1),
                    bomb_image,
                    font,
                );
            });

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
        })
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn compute_adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let width = width as f32;
        let height = height as f32;

        let max_width = window_width / width;
        let max_height = window_height / height;

        max_width.min(max_height).clamp(min, max)
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        tile_color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
    ) {
        // Tiles
        for (y, tile_row) in tile_map.map().iter().enumerate() {
            for (x, tile) in tile_row.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let mut cmd = parent.spawn_empty();

                // Create all of the components common between all tiles
                cmd.insert(SpriteBundle {
                    sprite: Sprite {
                        color: tile_color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    ..Default::default()
                });
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)));
                cmd.insert(coordinates);

                // Create tile-specific components
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..default()
                            });
                        });
                    }

                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { number: *v });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v,
                                font.clone(),
                                tile_size - tile_padding,
                            ));
                        });
                    }

                    Tile::Empty => (),
                }
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(number: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let text = number.to_string();
        let color = match number {
            1 => Color::WHITE,
            2 => Color::srgb_u8(135, 251, 106), // Green
            3 => Color::srgb_u8(222, 251, 106), // Yellow
            4 => Color::srgb_u8(251, 193, 106), // Orange
            _ => Color::srgb_u8(222, 106, 251), // Purple
        };

        let y_offset = -size / 10.;

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                justify: JustifyText::Center,
                ..default()
            },
            transform: Transform::from_xyz(0., y_offset, 1.),
            ..default()
        }
    }
}
