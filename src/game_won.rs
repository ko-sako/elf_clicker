use crate::{GameState, InternalState};
use macroquad::{
    audio::{PlaySoundParams, play_sound},
    prelude::*,
};

pub async fn game_won_update(internal_state: &mut InternalState) -> GameState {
    play_sound(
        &internal_state.game_won_bgm,
        PlaySoundParams {
            looped: true,
            volume: 0.3,
        },
    );
    loop {
        internal_state.timer += get_frame_time();

        draw_texture_ex(
            &internal_state.game_won_texture,
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

        draw_text("Ok, you win! I give up...", 20.0, 30.0, 28.0, BLACK);

        if internal_state.timer < 5.0 {
            next_frame().await
        } else {
            return GameState::Exit;
        }
    }
}
