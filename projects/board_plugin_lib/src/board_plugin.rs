use crate::{
    components::*,
    events::{BoardCompletedEvent, BombExplosionEvent, TileMarkEvent, TileTriggerEvent},
    resources::*,
    systems::*,
    util::*,
};
use bevy::{log, prelude::*, utils::HashMap};

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.running_state.clone()), Self::create_board);
        app.add_systems(OnExit(self.running_state.clone()), Self::cleanup_board);

        app.add_systems(
            FixedUpdate,
            (
                mouse_input,
                touch_input::<_, TouchInputTouchInputDeps>,
                read_tile_trigger_event,
                mark_tiles,
                uncover_tiles,
            )
                .run_if(in_state(self.running_state.clone())),
        );

        app.add_event::<BoardCompletedEvent>();
        app.add_event::<BombExplosionEvent>();
        app.add_event::<TileMarkEvent>();
        app.add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}

impl<T> BoardPlugin<T> {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Query<&Window>,
        board_assets: Res<BoardAssets>,
    ) {
        // Load assets

        // Create the tile map

        let window = windows.get_single().unwrap();

        let board_options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_map = TileMap::new_with_bombs(
            board_options.map_size.0,
            board_options.map_size.1,
            board_options.bomb_count,
        );
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_padding = board_options.tile_padding;
        let tile_size = match board_options.tile_size {
            TileSizeOption::Fixed(v) => v,
            TileSizeOption::Adaptive { min, max } => Self::compute_adaptive_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

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
        let mut safe_start_entity = None;
        let board_entity = {
            let mut board_entity_commands = commands.spawn_empty();
            board_entity_commands
                .insert(Name::new("Board"))
                .insert(Transform::from_translation(board_position))
                .insert(GlobalTransform::default())
                .insert(InheritedVisibility::default())
                .insert(TouchInterpretationComponent::default())
                .with_children(|parent| {
                    // Spawn background
                    parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.board_material.color,
                                custom_size: Some(board_size),
                                ..default()
                            },
                            texture: board_assets.board_material.texture.clone(),
                            transform: Transform::from_xyz(
                                board_size.x / 2.,
                                board_size.y / 2.,
                                0.,
                            ),
                            ..default()
                        })
                        .insert(Name::new("Background"));

                    // Spawn tiles
                    Self::spawn_tiles(
                        parent,
                        &tile_map,
                        tile_size,
                        tile_padding,
                        &board_assets,
                        &mut covered_tiles,
                        &mut safe_start_entity,
                    );
                });
            board_entity_commands.id()
        };

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            tile_padding,
            covered_tiles,
            entity: board_entity,
            marked_tiles: Vec::new(),
        });

        if board_options.safe_start {
            if let Some(entity) = safe_start_entity {
                commands.entity(entity).insert(Uncover);
            }
        }
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

    #[allow(clippy::too_many_arguments)]
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
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
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    texture: board_assets.tile_material.texture.clone(),
                    ..Default::default()
                });
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)));
                cmd.insert(coordinates);

                // Add covered sprites
                cmd.with_children(|parent| {
                    let mut entity_commands = parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                            color: board_assets.covered_tile_material.color,
                            ..default()
                        },
                        texture: board_assets.covered_tile_material.texture.clone(),
                        transform: Transform::from_xyz(0.0, 0., 2.),
                        ..default()
                    });
                    entity_commands.insert(Name::new("Tile Cover"));

                    let entity = entity_commands.id();
                    covered_tiles.insert(coordinates, entity);

                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                // Create tile-specific components
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: board_assets.bomb_material.color,
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..default()
                            });
                        });
                    }

                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { number: *v });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v,
                                board_assets,
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
    fn bomb_count_text_bundle(number: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
        let text = number.to_string();
        let color = board_assets.bomb_number_color(number);

        let y_offset = -size / 10.;

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font: board_assets.bomb_number_font.clone(),
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

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
