#[cfg(feature = "wasm")]
mod internal {
    use crate::config::{F32ClampConfig, GameColorConfig, U16Vec2Config, Vec2Config};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Default, Clone)]
    pub struct GameConfig {
        /// The root location to load assets from
        #[wasm_bindgen(skip)]
        pub asset_root: Option<String>,

        /// The number of bombs on the map
        #[wasm_bindgen(skip)]
        pub bomb_count: Option<u16>,

        /// The id selector of the canvas to render the game in
        #[wasm_bindgen(skip)]
        pub canvas_id_selector: Option<String>,

        /// The colors used by the game
        #[wasm_bindgen(skip)]
        pub color_config: Option<GameColorConfig>,

        /// The resolution of the game
        #[wasm_bindgen(skip)]
        pub resolution: Option<Vec2Config>,

        /// The number of tiles in the map
        #[wasm_bindgen(skip)]
        pub tile_count: Option<U16Vec2Config>,

        /// The size of the padding between tiles
        #[wasm_bindgen(skip)]
        pub tile_padding_size: Option<f32>,

        /// The minimum and maximum tile size
        #[wasm_bindgen(skip)]
        pub tile_size: Option<F32ClampConfig>,
    }

    #[wasm_bindgen]
    impl GameConfig {
        #[allow(clippy::too_many_arguments)]
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(
            asset_root: Option<String>,
            bomb_count: Option<u16>,
            canvas_id_selector: Option<String>,
            color_config: Option<GameColorConfig>,
            resolution: Option<Vec2Config>,
            tile_count: Option<U16Vec2Config>,
            tile_padding_size: Option<f32>,
            tile_size: Option<F32ClampConfig>,
        ) -> Self {
            Self {
                asset_root,
                bomb_count,
                canvas_id_selector,
                color_config,
                resolution,
                tile_count,
                tile_padding_size,
                tile_size,
            }
        }
    }
}

#[cfg(not(feature = "wasm"))]
mod internal {
    use crate::config::{F32ClampConfig, GameColorConfig, U16Vec2Config, Vec2Config};

    #[derive(Debug, Default, Clone)]
    pub struct GameConfig {
        /// The root location to load assets from
        pub asset_root: Option<String>,

        /// The number of bombs on the map
        pub bomb_count: Option<u16>,

        /// The id selector of the canvas to render the game in
        pub canvas_id_selector: Option<String>,

        /// The colors used by the game
        pub color_config: Option<GameColorConfig>,

        /// The resolution of the game
        pub resolution: Option<Vec2Config>,

        /// The number of tiles in the map
        pub tile_count: Option<U16Vec2Config>,

        /// The size of the padding between tiles
        pub tile_padding_size: Option<f32>,

        /// The minimum and maximum tile size
        pub tile_size: Option<F32ClampConfig>,
    }
}

pub use internal::*;
