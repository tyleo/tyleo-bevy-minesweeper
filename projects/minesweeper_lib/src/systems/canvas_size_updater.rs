use crate::{
    components::Coordinates,
    resources::{Board, BoardOptions, BoardPositionOption},
    util::get_canvas_size,
};
use bevy::{ecs::query::QueryEntityError, math::U16Vec2, prelude::*};

pub fn canvas_size_updater(
    mut board: ResMut<Board>,
    board_options: Option<Res<BoardOptions>>,
    coordinates: Query<&Coordinates>,
    mut windows: Query<&mut Window>,
    mut transforms: Query<&mut Transform>,
    mut sprites: Query<&mut Sprite>,
    mut texts: Query<Option<&mut Text>>,
) -> Result<(), QueryEntityError> {
    let canvas_size = get_canvas_size().unwrap();
    if board.canvas_size == canvas_size {
        return Ok(());
    }

    let mut window = windows.get_single_mut().unwrap();
    window.resolution.set(canvas_size.x, canvas_size.y);

    // Set the board canvas size
    board.canvas_size = canvas_size;

    let board_options = BoardOptions::optional_resource_or_default(board_options);

    let tile_padding = board_options.tile_padding;
    let tile_size = board_options.compute_tile_size(
        &board.canvas_size,
        U16Vec2::new(board.tile_map.width(), board.tile_map.height()),
    );

    let board_size = Vec2::new(
        board.tile_map.width() as f32 * tile_size,
        board.tile_map.height() as f32 * tile_size,
    );

    let board_position = match board_options.position {
        BoardPositionOption::Centered { offset } => {
            Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
        }
        BoardPositionOption::Custom(p) => p,
    };

    let mut board_transform = transforms.get_mut(board.entity)?;
    *board_transform = Transform::from_translation(board_position);

    let mut board_background_transform = transforms.get_mut(board.background_entity)?;
    *board_background_transform = Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.);

    let mut board_background_sprite = sprites.get_mut(board.background_entity)?;
    board_background_sprite.custom_size = Some(board_size);

    for entity in board.entities.iter() {
        let coordinates = coordinates.get(entity.root)?;

        let mut root_transform = transforms.get_mut(entity.root)?;
        *root_transform = Transform::from_xyz(
            (coordinates.x as f32 * tile_size) + (tile_size / 2.),
            (coordinates.y as f32 * tile_size) + (tile_size / 2.),
            1.,
        );

        let mut root_sprite = sprites.get_mut(entity.root)?;
        root_sprite.custom_size = Some(Vec2::splat(tile_size - tile_padding));

        let mut cover_sprite = sprites.get_mut(entity.cover)?;
        cover_sprite.custom_size = Some(Vec2::splat(tile_size - tile_padding));

        if let Some(kind) = entity.kind {
            if let Some(mut kind_text) = texts.get_mut(kind)? {
                let size = tile_size - tile_padding;

                let mut kind_transform = transforms.get_mut(kind)?;
                kind_transform.translation.y = -size / 10.;

                kind_text.sections[0].style.font_size = size;
            } else {
                let mut kind_sprite = sprites.get_mut(kind)?;
                kind_sprite.custom_size = Some(Vec2::splat(tile_size - tile_padding));
            }
        }
    }

    Ok(())
}
