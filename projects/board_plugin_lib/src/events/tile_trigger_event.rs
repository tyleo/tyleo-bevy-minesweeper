use crate::components::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Event)]
pub struct TileTriggerEvent(pub Coordinates);
