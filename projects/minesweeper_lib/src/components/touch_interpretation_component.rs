use crate::util::TouchInterpretationData;
use bevy::prelude::*;

/// Provides a component to understand the reason a player is touching the screen.
#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct TouchInterpretationComponent {
    pub data: Option<TouchInterpretationData>,
}
