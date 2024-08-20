use crate::resources::BoardColors;
#[cfg(feature = "wasm")]
mod internal {
    use crate::config::U8ColorConfig;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct GameColorConfig {
        #[wasm_bindgen(skip)]
        pub background_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub padding_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub unknown_tile_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub revealed_tile_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub highlighted_tile_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub flag_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub bomb_color: Option<U8ColorConfig>,

        #[wasm_bindgen(skip)]
        pub number_colors: Option<Vec<U8ColorConfig>>,
    }

    #[wasm_bindgen]
    impl GameColorConfig {
        #[allow(clippy::too_many_arguments)]
        #[wasm_bindgen(constructor)]
        pub fn wasm_constructor(
            background_color: Option<U8ColorConfig>,
            padding_color: Option<U8ColorConfig>,
            unknown_tile_color: Option<U8ColorConfig>,
            revealed_tile_color: Option<U8ColorConfig>,
            highlighted_tile_color: Option<U8ColorConfig>,
            flag_color: Option<U8ColorConfig>,
            bomb_color: Option<U8ColorConfig>,
            number_colors: Option<Vec<U8ColorConfig>>,
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
}

#[cfg(not(feature = "wasm"))]
mod internal {
    use crate::config::U8ColorConfig;

    #[derive(Debug, Clone)]
    pub struct GameColorConfig {
        pub background_color: Option<U8ColorConfig>,

        pub padding_color: Option<U8ColorConfig>,

        pub unknown_tile_color: Option<U8ColorConfig>,

        pub revealed_tile_color: Option<U8ColorConfig>,

        pub highlighted_tile_color: Option<U8ColorConfig>,

        pub flag_color: Option<U8ColorConfig>,

        pub bomb_color: Option<U8ColorConfig>,

        pub number_colors: Option<Vec<U8ColorConfig>>,
    }
}

pub use internal::*;

impl From<GameColorConfig> for BoardColors {
    fn from(value: GameColorConfig) -> Self {
        let default = BoardColors::default();

        Self {
            background_color: value
                .background_color
                .map(|c| c.into())
                .unwrap_or(default.background_color),

            padding_color: value
                .padding_color
                .map(|c| c.into())
                .unwrap_or(default.padding_color),

            unknown_tile_color: value
                .unknown_tile_color
                .map(|c| c.into())
                .unwrap_or(default.unknown_tile_color),

            revealed_tile_color: value
                .revealed_tile_color
                .map(|c| c.into())
                .unwrap_or(default.revealed_tile_color),

            highlighted_tile_color: value
                .highlighted_tile_color
                .map(|c| c.into())
                .unwrap_or(default.highlighted_tile_color),

            flag_color: value
                .flag_color
                .map(|c| c.into())
                .unwrap_or(default.flag_color),

            bomb_color: value
                .bomb_color
                .map(|c| c.into())
                .unwrap_or(default.bomb_color),

            number_colors: value
                .number_colors
                .map(|c| c.into_iter().map(|c| c.into()).collect())
                .unwrap_or(default.number_colors),
        }
    }
}
