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
