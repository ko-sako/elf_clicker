use std::time::Duration;

use macroquad::{
    audio::{PlaySoundParams, load_sound, play_sound},
    prelude::*,
};

use crate::{GameState, InternalState};

pub async fn main_menu_update(_internal_state: &mut InternalState) -> GameState {
    let bg = load_texture("assets/background.png").await.unwrap();
    let start_game_sound = load_sound("assets/start_game_sound.ogg").await.unwrap();
    let main_menu_music = load_sound("assets/menu_bgm.ogg").await.unwrap();

    let start_game_button = Rect::new(
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 80.0,
        200.0,
        60.0,
    );

    play_sound(
        &main_menu_music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    loop {
        clear_background(BLACK);

        draw_texture_ex(
            &bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        let (mx, my) = mouse_position();
        let hovered = start_game_button.contains(vec2(mx, my));

        draw_rectangle(
            start_game_button.x,
            start_game_button.y,
            start_game_button.w,
            start_game_button.h,
            if hovered { DARKGRAY } else { GRAY },
        );

        draw_text(
            "PLAY",
            start_game_button.x + 60.0,
            start_game_button.y + 40.0,
            40.0,
            WHITE,
        );

        // click `PLAY` button
        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            play_sound(
                &start_game_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            // wait a bit for listening the sound :)
            std::thread::sleep(Duration::from_millis(700));
            return GameState::InGame;
        }

        next_frame().await
    }
}
