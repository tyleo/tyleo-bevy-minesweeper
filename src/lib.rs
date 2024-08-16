mod shared;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::shared;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct F32Bounds(pub f32, pub f32);

    impl From<F32Bounds> for (f32, f32) {
        fn from(value: F32Bounds) -> Self {
            (value.0, value.1)
        }
    }

    #[wasm_bindgen]
    pub struct U16Bounds(pub u16, pub u16);

    impl From<U16Bounds> for (u16, u16) {
        fn from(value: U16Bounds) -> Self {
            (value.0, value.1)
        }
    }

    #[wasm_bindgen]
    pub fn run(
        canvas_id_selector: String,
        canvas_size: F32Bounds,
        num_tiles: Option<U16Bounds>,
        tile_size: Option<F32Bounds>,
        tile_padding: Option<f32>,
    ) {
        shared::run(
            Some(canvas_id_selector),
            Some(canvas_size.into()),
            num_tiles.map(|num_tiles| num_tiles.into()),
            tile_size.map(|tile_size| tile_size.into()),
            tile_padding,
        );
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
