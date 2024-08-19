use crate::wasm::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(
    canvas_id_selector: String,
    canvas_size: F32Pair,
    num_tiles: Option<U16Pair>,
    tile_size: Option<F32Pair>,
    tile_padding: Option<f32>,
) {
    crate::run(
        Some(canvas_id_selector),
        Some(canvas_size.into()),
        num_tiles.map(|num_tiles| num_tiles.into()),
        tile_size.map(|tile_size| tile_size.into()),
        tile_padding,
    );
}
