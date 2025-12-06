use crate::{GameState, InternalState};

pub fn game_lost_update(_internal_state: &mut InternalState) -> GameState {
    GameState::Exit
}
