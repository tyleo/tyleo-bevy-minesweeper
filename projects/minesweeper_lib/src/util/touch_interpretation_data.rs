use bevy::prelude::*;

/// Provides data to understand the reason a player is touching the screen.
#[derive(Clone, Debug, Reflect)]
pub struct TouchInterpretationData {
    pub x: u16,
    pub y: u16,
    pub timestamp: f32,
    pub cover_entity: Entity,
}
