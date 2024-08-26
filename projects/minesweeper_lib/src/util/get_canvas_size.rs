use super::CANVAS_SIZE;
use bevy::prelude::*;

pub fn get_canvas_size() -> Option<Vec2> {
    *CANVAS_SIZE.lock().unwrap()
}
