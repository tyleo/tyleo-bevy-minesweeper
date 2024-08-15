mod shared;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::shared;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn run(canvas_id_selector: String, width: f32, height: f32) {
        shared::run(Some(canvas_id_selector), Some((width, height)));
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
