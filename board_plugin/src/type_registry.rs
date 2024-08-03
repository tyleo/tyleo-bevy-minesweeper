use crate::{components::*, resources::*, util::RegisterTypes};
use bevy::app::App;

pub struct TypeRegistry;

impl RegisterTypes for TypeRegistry {
    fn register_types(app: &mut App) {
        // Components
        app.register_type::<Board>();
        app.register_type::<Bomb>();
        app.register_type::<BombNeighbor>();
        app.register_type::<Coordinates>();
        app.register_type::<Uncover>();

        // Resources
        app.register_type::<BoardOptions>();
    }
}
