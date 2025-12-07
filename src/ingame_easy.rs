use macroquad::{
    audio::{PlaySoundParams, play_sound, stop_sound},
    prelude::*,
    rand::gen_range,
};

use crate::{GameState, InternalState};

pub async fn in_game_easy_update(internal_state: &mut InternalState) -> GameState {
    play_sound(
        &internal_state.ingame_bgm,
        PlaySoundParams {
            looped: true,
            volume: 0.2,
        },
    );
    loop {
        draw_texture_ex(
            &internal_state.ingame_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // shelf
        // let shelf_height = 40.0;
        let shelf_y = screen_height() * 0.75;
        // draw_rectangle(0.0, shelf_y, screen_width(), shelf_height, BROWN);

        // elf face
        let elf_r = 60.0;

        internal_state.timer += get_frame_time();
        if internal_state.timer > 0.02 {
            internal_state.frame_index =
                (internal_state.frame_index + 1) % internal_state.frames.len();
            internal_state.timer = 0.00;
        }

        // elf1
        draw_texture(
            &internal_state.frames[internal_state.frame_index],
            internal_state.elf.x - &internal_state.frames[internal_state.frame_index].width() / 2.0,
            internal_state.elf.y
                - &internal_state.frames[internal_state.frame_index].height() / 2.0,
            WHITE,
        );

        draw_circle(
            internal_state.elf.x,
            internal_state.elf.y,
            elf_r + 8.0,
            Color {
                r: 255.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            },
        );

        draw_texture(
            &internal_state.mouse_texture,
            mouse_position().0 - 94.0,
            mouse_position().1 - 86.0,
            WHITE,
        );

        // click logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let distance_from_elf = vec2(mouse_x, mouse_y).distance(internal_state.elf);

            if distance_from_elf <= elf_r + 8.0 {
                // hit
                internal_state.clicks += 1;
                play_sound(
                    &internal_state.gun_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.0,
                    },
                );
                play_sound(
                    &internal_state.hit_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.0,
                    },
                );
            } else {
                internal_state.misses += 1;
                play_sound(
                    &internal_state.gun_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.0,
                    },
                );
            }
            internal_state.msg = format!(
                "You hit my face: {}, Missed: {}",
                internal_state.clicks, internal_state.misses
            );
            internal_state.elf.x = gen_range(30.0, screen_width() - 30.0);
            internal_state.elf.y = gen_range(50.0, shelf_y - elf_r - 10.0);
        }

        draw_text(&internal_state.msg, 20.0, 30.0, 28.0, WHITE);

        if internal_state.clicks == 5 {
            stop_sound(&internal_state.ingame_bgm);
            return GameState::GameWon;
        } else if internal_state.misses == 5 {
            internal_state.timer = 0.0;
            stop_sound(&internal_state.ingame_bgm);
            return GameState::GameLost;
        }
        next_frame().await
    }
}
