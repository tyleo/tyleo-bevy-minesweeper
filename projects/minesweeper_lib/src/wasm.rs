use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct F32Pair(pub f32, pub f32);

#[wasm_bindgen]
impl F32Pair {
    #[wasm_bindgen(constructor)]
    pub fn new(first: f32, second: f32) -> F32Pair {
        F32Pair(first, second)
    }
}

impl From<F32Pair> for (f32, f32) {
    fn from(value: F32Pair) -> Self {
        (value.0, value.1)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct U16Pair(pub u16, pub u16);

#[wasm_bindgen]
impl U16Pair {
    #[wasm_bindgen(constructor)]
    pub fn new(first: u16, second: u16) -> U16Pair {
        U16Pair(first, second)
    }
}

impl From<U16Pair> for (u16, u16) {
    fn from(value: U16Pair) -> Self {
        (value.0, value.1)
    }
}

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
