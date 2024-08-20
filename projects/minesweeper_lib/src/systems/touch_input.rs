use crate::{components::*, events::*, resources::*, util::*};
use bevy::{
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    log,
    prelude::*,
};

pub enum TouchState {
    Started,
    Finished,
    Other,
}

pub trait TouchInputDeps<E> {
    fn get_position_from_event(event: &E) -> Option<Vec2>;

    fn get_touch_state(event: &E) -> TouchState;

    fn auto_update() -> bool;
}

pub struct TouchInputTouchInputDeps;

impl TouchInputDeps<TouchInput> for TouchInputTouchInputDeps {
    fn get_position_from_event(event: &TouchInput) -> Option<Vec2> {
        Some(event.position)
    }

    fn get_touch_state(event: &TouchInput) -> TouchState {
        match event.phase {
            TouchPhase::Started => TouchState::Started,
            TouchPhase::Ended => TouchState::Finished,
            TouchPhase::Canceled => TouchState::Finished,
            TouchPhase::Moved => TouchState::Other,
        }
    }

    fn auto_update() -> bool {
        false
    }
}

pub struct MouseButtonInputTouchInputDeps;

impl TouchInputDeps<MouseButtonInput> for MouseButtonInputTouchInputDeps {
    fn get_position_from_event(_event: &MouseButtonInput) -> Option<Vec2> {
        None
    }

    fn get_touch_state(event: &MouseButtonInput) -> TouchState {
        match event.button {
            MouseButton::Left => match event.state {
                ButtonState::Pressed => TouchState::Started,
                ButtonState::Released => TouchState::Finished,
            },
            _ => TouchState::Other,
        }
    }

    fn auto_update() -> bool {
        true
    }
}

#[allow(clippy::too_many_arguments)]
pub fn touch_input<E: Event, D: TouchInputDeps<E>>(
    mut commands: Commands,
    windows: Query<&Window>,
    mut touch_interpreters: Query<&mut TouchInterpretationComponent>,
    mut cover_transforms: Query<&mut Transform, With<Cover>>,
    board: Res<Board>,
    board_assets: Res<BoardAssets>,
    time: Res<Time<Fixed>>,
    mut touch_event_reader: EventReader<E>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_single().unwrap();
    let mut touch_interpreter = touch_interpreters.get_single_mut().unwrap();

    let window_position = window.cursor_position();
    let tile_size = board.tile_size;
    let tile_padding = board.tile_padding;

    for event in touch_event_reader.read() {
        let touch_state = D::get_touch_state(event);

        let event_position = D::get_position_from_event(event);
        let position = event_position.or(window_position);
        let tile_coordinates = position.and_then(|position| board.mouse_position(window, position));

        if let Some(tile_coordinates) = tile_coordinates {
            // Only highlight the tile if it is covered
            let z = if board.covered_tiles.contains_key(&tile_coordinates) {
                3.5
            } else {
                0.
            };

            match touch_state {
                TouchState::Started => {
                    log::trace!("Touch started: {}", tile_coordinates);

                    // Create an entity to highlight the tile
                    let cover_entity = commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.pending_tile_material.color,
                                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                ..default()
                            },
                            texture: board_assets.pending_tile_material.texture.clone(),
                            transform: Transform::from_xyz(
                                (tile_coordinates.x as f32 * tile_size) + (tile_size / 2.),
                                (tile_coordinates.y as f32 * tile_size) + (tile_size / 2.),
                                z,
                            ),
                            ..default()
                        })
                        .insert(Cover)
                        .id();
                    commands.entity(board.entity).push_children(&[cover_entity]);
                    touch_interpreter.data = Some(TouchInterpretationData {
                        x: tile_coordinates.x,
                        y: tile_coordinates.y,
                        timestamp: time.elapsed_seconds(),
                        cover_entity,
                    });
                }
                TouchState::Finished => {
                    log::trace!("Touch ended: {}", tile_coordinates);

                    if let Some(touch_interpretation_data) =
                        std::mem::take(&mut touch_interpreter.data)
                    {
                        let timestamp = time.elapsed_seconds();
                        let timespan = timestamp - touch_interpretation_data.timestamp;
                        if timespan < 0.5 {
                            log::info!("Trying to uncover tile on {}", tile_coordinates);
                            tile_trigger_event_writer.send(TileTriggerEvent(tile_coordinates));
                        }

                        // Destroy the entity which covers the tile
                        commands
                            .entity(touch_interpretation_data.cover_entity)
                            .despawn_recursive();
                    }
                }
                TouchState::Other => {
                    if let Some(ref mut touch_interpretation_data) = &mut touch_interpreter.data {
                        if touch_interpretation_data.x != tile_coordinates.x
                            || touch_interpretation_data.y != tile_coordinates.y
                        {
                            touch_interpretation_data.x = tile_coordinates.x;
                            touch_interpretation_data.y = tile_coordinates.y;
                            touch_interpretation_data.timestamp = time.elapsed_seconds();
                        }
                    }

                    for mut cover_transform in cover_transforms.iter_mut() {
                        cover_transform.translation = Vec3::new(
                            (tile_coordinates.x as f32 * tile_size) + (tile_size / 2.),
                            (tile_coordinates.y as f32 * tile_size) + (tile_size / 2.),
                            z,
                        );
                    }
                }
            }
        }
    }

    if D::auto_update() {
        let tile_coordinates =
            window_position.and_then(|position| board.mouse_position(window, position));

        if let Some(tile_coordinates) = tile_coordinates {
            if let Some(ref mut touch_interpretation_data) = &mut touch_interpreter.data {
                if touch_interpretation_data.x != tile_coordinates.x
                    || touch_interpretation_data.y != tile_coordinates.y
                {
                    touch_interpretation_data.x = tile_coordinates.x;
                    touch_interpretation_data.y = tile_coordinates.y;
                    touch_interpretation_data.timestamp = time.elapsed_seconds();
                }
            }

            // Only highlight the tile if it is covered
            let z = if board.covered_tiles.contains_key(&tile_coordinates) {
                3.5
            } else {
                0.
            };

            for mut cover_transform in cover_transforms.iter_mut() {
                cover_transform.translation = Vec3::new(
                    (tile_coordinates.x as f32 * tile_size) + (tile_size / 2.),
                    (tile_coordinates.y as f32 * tile_size) + (tile_size / 2.),
                    z,
                );
            }
        }
    }

    touch_interpreter.data =
        if let Some(touch_interpretation_data) = std::mem::take(&mut touch_interpreter.data) {
            let timestamp = time.elapsed_seconds();
            let timespan = timestamp - touch_interpretation_data.timestamp;
            if timespan >= 0.5 {
                let tile_coordinates = Coordinates {
                    x: touch_interpretation_data.x,
                    y: touch_interpretation_data.y,
                };

                commands
                    .entity(touch_interpretation_data.cover_entity)
                    .despawn_recursive();

                log::info!("Trying to mark tile on {}", tile_coordinates);
                tile_mark_event_writer.send(TileMarkEvent(tile_coordinates));

                None
            } else {
                Some(touch_interpretation_data)
            }
        } else {
            None
        };
}
