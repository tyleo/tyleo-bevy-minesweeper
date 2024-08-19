use wasm_bindgen::prelude::*;

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
