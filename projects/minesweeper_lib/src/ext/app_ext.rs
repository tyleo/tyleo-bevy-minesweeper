use crate::util::*;
use bevy::app::App;

pub trait AppExt {
    fn register_types<TTypes: RegisterTypes>(&mut self, _types: TTypes);
}

impl AppExt for App {
    fn register_types<TTypes: RegisterTypes>(&mut self, _types: TTypes) {
        TTypes::register_types(self)
    }
}
