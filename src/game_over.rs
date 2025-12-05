use crate::{GameState, InternalState};

pub fn game_over_update(internal_state: &mut InternalState) -> GameState {
    GameState::Exit
}
