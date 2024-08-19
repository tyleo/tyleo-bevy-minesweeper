use crate::{components::*, events::*, resources::*};
use bevy::{log, prelude::*};

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut board_completed_event_writer: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_writer: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(**parent) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        // We remove the entity from the board covered tile map
        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover an already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }

        if board.is_completed() {
            log::info!("Board completed");
            board_completed_event_writer.send(BoardCompletedEvent);
        }

        if bomb.is_some() {
            log::info!("Boom !");
            bomb_explosion_event_writer.send(BombExplosionEvent);
        }
        // If the tile is empty..
        else if bomb_counter.is_none() {
            // .. We propagate the uncovering by adding the `Uncover` component to adjacent tiles
            // which will then be removed next frame
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
