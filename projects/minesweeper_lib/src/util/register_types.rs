use bevy::app::App;

pub trait RegisterTypes {
    fn register_types(app: &mut App);
}
