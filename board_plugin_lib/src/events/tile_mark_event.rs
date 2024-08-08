use crate::components::*;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Event)]
pub struct TileMarkEvent(pub Coordinates);
