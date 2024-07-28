pub mod components;
pub mod ext;
pub mod resources;
pub mod util;

use bevy::log;
use bevy::prelude::*;
use resources::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board() {
        let tile_map = TileMap::new_with_bombs(20, 20, 40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
