use crate::{events::*, resources::*};
use bevy::{input::touch::TouchPhase, log, prelude::*};

pub fn touch_input(
    windows: Query<&Window>,
    board: Res<Board>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    // mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_single().unwrap();

    for event in touch_event_reader.read() {
        match event.phase {
            TouchPhase::Started => {
                let touch_position = event.position;
                log::trace!("Touch pressed: {}", touch_position);

                let tile_coordinates = board.mouse_position(window, touch_position);
                if let Some(tile_coordinates) = tile_coordinates {
                    log::info!("Trying to uncover tile on {}", tile_coordinates);
                    tile_trigger_event_writer.send(TileTriggerEvent(tile_coordinates));
                    // match event.button {
                    //     MouseButton::Left => {
                    //         log::info!("Trying to uncover tile on {}", tile_coordinates);
                    //         tile_trigger_event_writer.send(TileTriggerEvent(tile_coordinates));
                    //     }
                    //     MouseButton::Right => {
                    //         log::info!("Trying to mark tile on {}", tile_coordinates);
                    //         tile_mark_event_writer.send(TileMarkEvent(tile_coordinates));
                    //     }
                    //     _ => {}
                    // }
                }
            }
            _ => {}
        }
    }
}
