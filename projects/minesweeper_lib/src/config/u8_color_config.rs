use bevy::prelude::*;

#[cfg(feature = "wasm")]
mod internal {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct U8ColorConfig {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    #[wasm_bindgen]
    impl U8ColorConfig {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(red: u8, green: u8, blue: u8) -> Self {
            Self { red, green, blue }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    #[derive(Debug, Clone)]
    pub struct U8ColorConfig {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }
}

pub use internal::*;

impl From<U8ColorConfig> for Color {
    fn from(value: U8ColorConfig) -> Self {
        Color::srgb_u8(value.red, value.green, value.blue)
    }
}
