use bevy::{log, prelude::*};

use crate::{
    config::Vec2Config,
    util::{get_canvas_size, set_canvas_size},
};

pub fn update_canvas_size_from_window(windows: Query<&Window>) {
    let window = windows.get_single().unwrap();
    let size = window.resolution.size();

    let canvas_size = get_canvas_size().unwrap();

    if size == canvas_size {
        return;
    }

    log::info!(
        "Updating canvas_size: {{ 'original': {}, 'new': {} }}",
        canvas_size,
        size
    );

    set_canvas_size(Vec2Config {
        x: size.x,
        y: size.y,
    });
}
