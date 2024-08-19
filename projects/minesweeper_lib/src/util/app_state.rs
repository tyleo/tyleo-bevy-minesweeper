use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Loading,
    Loaded,
    Out,
    InGame,
}
