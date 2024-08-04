use crate::Board;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::*,
};

pub fn input_handling(
    windows: Query<&Window>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.get_single().unwrap();

    for event in button_evr.read() {
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
                    }
                    MouseButton::Right => {
                        log::info!("Trying to mark tile on {}", tile_coordinates);
                    }
                    _ => {}
                }
            }
        }
    }
}
