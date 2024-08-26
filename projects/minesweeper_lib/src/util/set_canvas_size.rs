use super::CANVAS_SIZE;
use crate::config::Vec2Config;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn set_canvas_size(canvas_size: Vec2Config) {
    let mut locked_canvas_size = CANVAS_SIZE.lock().unwrap();
    *locked_canvas_size = Some(canvas_size.into());
}
