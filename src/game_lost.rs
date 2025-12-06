use macroquad::prelude::*;

use crate::{GameState, InternalState};

pub fn game_lost_update(internal_state: &mut InternalState) -> GameState {
    internal_state.timer += get_frame_time();

    draw_texture_ex(
        &internal_state.game_lost_texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );

    draw_text(
        "Game over! I'm telling Santa you've been naughty!",
        20.0,
        30.0,
        28.0,
        BLACK,
    );

    if internal_state.timer < 3.0 {
        GameState::GameLost
    } else {
        GameState::Exit
    }
}
