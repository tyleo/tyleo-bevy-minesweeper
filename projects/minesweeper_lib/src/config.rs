#[cfg(feature = "wasm")]
mod internal {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct U16Vec2 {
        pub x: u16,
        pub y: u16,
    }

    #[wasm_bindgen]
    impl U16Vec2 {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(x: u16, y: u16) -> Self {
            Self { x, y }
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }

    #[wasm_bindgen]
    impl Vec2 {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct U8Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    #[wasm_bindgen]
    impl U8Color {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(r: u8, b: u8, g: u8) -> Self {
            Self { r, b, g }
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct ColorConfig {
        #[wasm_bindgen(skip)]
        pub background_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub padding_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub unknown_tile_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub revealed_tile_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub highlighted_tile_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub flag_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub bomb_color: Option<U8Color>,

        #[wasm_bindgen(skip)]
        pub number_colors: Option<Vec<U8Color>>,
    }

    #[wasm_bindgen]
    impl ColorConfig {
        #[allow(clippy::too_many_arguments)]
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(
            background_color: Option<U8Color>,
            padding_color: Option<U8Color>,
            unknown_tile_color: Option<U8Color>,
            revealed_tile_color: Option<U8Color>,
            highlighted_tile_color: Option<U8Color>,
            flag_color: Option<U8Color>,
            bomb_color: Option<U8Color>,
            number_colors: Option<Vec<U8Color>>,
        ) -> Self {
            Self {
                background_color,
                padding_color,
                unknown_tile_color,
                revealed_tile_color,
                highlighted_tile_color,
                flag_color,
                bomb_color,
                number_colors,
            }
        }
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct Config {
        /// The number of bombs on the map
        #[wasm_bindgen(skip)]
        pub bomb_count: Option<u16>,

        /// The number of tiles in the map
        #[wasm_bindgen(skip)]
        pub tile_count: Option<U16Vec2>,

        /// The size of the padding between tiles
        #[wasm_bindgen(skip)]
        pub tile_padding_size: Option<f32>,

        /// The minimum and maximum tile size
        #[wasm_bindgen(skip)]
        pub tile_size: Option<Vec2>,

        /// The colors used by the game
        #[wasm_bindgen(skip)]
        pub color_config: Option<ColorConfig>,
    }

    #[wasm_bindgen]
    impl Config {
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(
            bomb_count: Option<u16>,
            tile_count: Option<U16Vec2>,
            tile_padding_size: Option<f32>,
            tile_size: Option<Vec2>,
            color_config: Option<ColorConfig>,
        ) -> Self {
            Self {
                bomb_count,
                tile_count,
                tile_padding_size,
                tile_size,
                color_config,
            }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    #[derive(Debug, Clone)]
    pub struct U16Vec2 {
        pub x: u16,
        pub y: u16,
    }

    #[derive(Debug, Clone)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Debug, Clone)]
    pub struct U8Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    #[derive(Debug, Clone)]
    pub struct ColorConfig {
        pub background_color: Option<U8Color>,
        pub padding_color: Option<U8Color>,
        pub unknown_tile_color: Option<U8Color>,
        pub revealed_tile_color: Option<U8Color>,
        pub highlighted_tile_color: Option<U8Color>,
        pub flag_color: Option<U8Color>,
        pub bomb_color: Option<U8Color>,
        pub number_colors: Option<Vec<U8Color>>,
    }

    #[derive(Debug, Clone)]
    pub struct Config {
        /// The number of bombs on the map
        pub bomb_count: Option<u16>,

        /// The number of tiles in the map
        pub tile_count: Option<U16Vec2>,

        /// The size of the padding between tiles
        pub tile_padding_size: Option<f32>,

        /// The minimum and maximum tile size
        pub tile_size: Option<Vec2>,

        /// The colors used by the game
        pub color_config: Option<ColorConfig>,
    }
}

pub use internal::*;
