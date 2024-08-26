use bevy::prelude::*;
use std::sync::Mutex;

pub(super) static CANVAS_SIZE: Mutex<Option<Vec2>> = Mutex::new(None);
