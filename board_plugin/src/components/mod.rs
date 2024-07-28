mod coordinates;

pub use coordinates::*;

use crate::util::RegisterTypes;
use bevy::app::App;

pub struct Components;

impl RegisterTypes for Components {
    fn register_types(app: &mut App) {
        app.register_type::<Coordinates>();
    }
}
