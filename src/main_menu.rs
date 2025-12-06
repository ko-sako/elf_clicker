use crate::{GameState, InternalState};

pub fn main_menu_update(_internal_state: &mut InternalState) -> GameState {
    // Hey Kayo, this is where you can display a menu.

    // return the InGame state when you want to transition, otherwise
    // return GameState::MainMenu
    GameState::InGame
}
