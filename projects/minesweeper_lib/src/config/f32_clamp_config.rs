#[cfg(feature = "wasm")]
mod internal {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct F32ClampConfig {
        pub min: f32,
        pub max: f32,
    }

    #[wasm_bindgen]
    impl F32ClampConfig {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(min: f32, max: f32) -> Self {
            Self { min, max }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    #[derive(Debug, Clone)]
    pub struct F32ClampConfig {
        pub min: f32,
        pub max: f32,
    }
}

pub use internal::*;
