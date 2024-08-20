use bevy::prelude::*;

#[cfg(feature = "wasm")]
mod internal {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct Vec2Config {
        pub x: f32,
        pub y: f32,
    }

    #[wasm_bindgen]
    impl Vec2Config {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    #[derive(Debug, Clone)]
    pub struct Vec2Config {
        pub x: f32,
        pub y: f32,
    }
}

pub use internal::*;

impl From<Vec2Config> for Vec2 {
    fn from(value: Vec2Config) -> Self {
        Vec2::new(value.x, value.y)
    }
}
