use crate::{events::*, resources::*};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::*,
};

pub fn mouse_input(
    windows: Query<&Window>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_single().unwrap();

    for event in button_event_reader.read() {
        let button_state = event.state;
        // Early out if the button is not pressed
        if button_state != ButtonState::Pressed {
            continue;
        }

        let cursor_position = window.cursor_position();
        if let Some(cursor_position) = cursor_position {
            log::trace!(
                "Mouse button pressed: {:?} at {}",
                event.button,
                cursor_position
            );

            let tile_coordinates = board.mouse_position(window, cursor_position);
            if let Some(tile_coordinates) = tile_coordinates {
                match event.button {
                    MouseButton::Left => {
                        log::info!("Trying to uncover tile on {}", tile_coordinates);
                        tile_trigger_event_writer.send(TileTriggerEvent(tile_coordinates));
                    }
                    MouseButton::Right => {
                        log::info!("Trying to mark tile on {}", tile_coordinates);
                        tile_mark_event_writer.send(TileMarkEvent(tile_coordinates));
                    }
                    _ => {}
                }
            }
        }
    }
}
