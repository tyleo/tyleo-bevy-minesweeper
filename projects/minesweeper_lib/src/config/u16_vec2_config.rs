#[cfg(feature = "wasm")]
mod internal {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct U16Vec2Config {
        pub x: u16,
        pub y: u16,
    }

    #[wasm_bindgen]
    impl U16Vec2Config {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(x: u16, y: u16) -> Self {
            Self { x, y }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    #[derive(Debug, Clone)]
    pub struct U16Vec2Config {
        pub x: u16,
        pub y: u16,
    }
}

pub use internal::*;
