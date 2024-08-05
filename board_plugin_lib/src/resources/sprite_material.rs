use bevy::prelude::*;

/// Material of a `Sprite` with a texture and color
#[derive(Debug, Clone, Reflect)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: Handle::default(),
        }
    }
}
