use crate::{GameState, InternalState};

pub fn game_won_update(_internal_state: &mut InternalState) -> GameState {
    GameState::Exit
}
