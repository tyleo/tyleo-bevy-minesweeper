use crate::{
    components::{Coordinates, TouchInterpretationComponent},
    events::*,
    resources::*,
    util::TouchInterpretationData,
};
use bevy::{input::touch::TouchPhase, log, prelude::*};

pub fn touch_input(
    mut commands: Commands,
    windows: Query<&Window>,
    mut touch_interpreters: Query<&mut TouchInterpretationComponent>,
    board: Res<Board>,
    board_assets: Res<BoardAssets>,
    time: Res<Time<Fixed>>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_single().unwrap();
    let mut touch_interpreter = touch_interpreters.get_single_mut().unwrap();

    for event in touch_event_reader.read() {
        let touch_position = event.position;
        let tile_coordinates = board.mouse_position(window, touch_position);

        let clear = if let Some(ref touch_interpretation_data) = touch_interpreter.data {
            if let Some(tile_coordinates) = tile_coordinates {
                if tile_coordinates.x != touch_interpretation_data.x
                    || tile_coordinates.y != touch_interpretation_data.y
                {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if clear {
            touch_interpreter.data = None;
        }

        match event.phase {
            TouchPhase::Started => {
                log::trace!("Touch started: {}", touch_position);

                let tile_coordinates = board.mouse_position(window, touch_position);
                if let Some(tile_coordinates) = tile_coordinates {
                    let tile_size = board.tile_size;
                    let tile_padding = board.tile_padding;

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
                                3.5,
                            ),
                            ..default()
                        })
                        .id();
                    commands.entity(board.entity).push_children(&[cover_entity]);
                    touch_interpreter.data = Some(TouchInterpretationData {
                        x: tile_coordinates.x,
                        y: tile_coordinates.y,
                        timestamp: time.elapsed_seconds(),
                        cover_entity,
                    });
                }
            }
            TouchPhase::Ended => {
                log::trace!("Touch ended: {}", touch_position);

                if let Some(ref touch_interpretation_data) = touch_interpreter.data {
                    let tile_coordinates = board.mouse_position(window, touch_position);
                    if let Some(tile_coordinates) = tile_coordinates {
                        let timestamp = time.elapsed_seconds();
                        let timespan = timestamp - touch_interpretation_data.timestamp;
                        if timespan < 0.5 {
                            log::info!("Trying to uncover tile on {}", tile_coordinates);
                            tile_trigger_event_writer.send(TileTriggerEvent(tile_coordinates));
                        }
                    }

                    commands
                        .entity(touch_interpretation_data.cover_entity)
                        .despawn_recursive();
                }

                touch_interpreter.data = None;
            }
            _ => {}
        }
    }

    let mark_tile = if let Some(ref touch_interpretation_data) = touch_interpreter.data {
        let timestamp = time.elapsed_seconds();
        let timespan = timestamp - touch_interpretation_data.timestamp;
        timespan >= 0.5
    } else {
        false
    };

    if mark_tile {
        let touch_interpreter_data = std::mem::take(&mut touch_interpreter.data).unwrap();
        let tile_coordinates = Coordinates {
            x: touch_interpreter_data.x,
            y: touch_interpreter_data.y,
        };
        commands
            .entity(touch_interpreter_data.cover_entity)
            .despawn_recursive();
        log::info!("Trying to mark tile on {}", tile_coordinates);
        tile_mark_event_writer.send(TileMarkEvent(tile_coordinates));
    }
}
