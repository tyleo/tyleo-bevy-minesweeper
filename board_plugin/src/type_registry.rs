use crate::{components::*, resources::*, util::RegisterTypes};
use bevy::app::App;

pub struct TypeRegistry;

impl RegisterTypes for TypeRegistry {
    fn register_types(app: &mut App) {
        // Components
        app.register_type::<Coordinates>();

        // Resources
        app.register_type::<BoardOptions>();
    }
}
