use macroquad::{prelude::*, rand::gen_range};

use crate::{GameState, InternalState};

pub fn in_game_update(internal_state: &mut InternalState) -> GameState {
    // shelf
    let shelf_height = 40.0;
    let shelf_y = screen_height() * 0.75;
    draw_rectangle(0.0, shelf_y, screen_width(), shelf_height, BROWN);

    // elf face
    let elf_r = 22.0;

    internal_state.timer += get_frame_time();
    if internal_state.timer > 0.02 {
        internal_state.frame_index = (internal_state.frame_index + 1) % internal_state.frames.len();
        internal_state.timer = 0.00;
    }

    draw_texture(
        &internal_state.frames[internal_state.frame_index],
        internal_state.elf.x - 250.0,
        internal_state.elf.y - 95.0,
        WHITE,
    );

    // click logic
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let distance_from_elf = vec2(mouse_x, mouse_y).distance(internal_state.elf);
        if distance_from_elf <= elf_r + 8.0 {
            // hit
            internal_state.clicks += 1;
        } else {
            internal_state.misses += 1;
        }
        internal_state.msg = format!(
            "You hit my face: {}, Missed: {}",
            internal_state.clicks, internal_state.misses
        );
        internal_state.elf.x = gen_range(30.0, screen_width() - 30.0);
        internal_state.elf.y = gen_range(50.0, shelf_y - elf_r - 10.0);
    }

    draw_text(&internal_state.msg, 20.0, 30.0, 28.0, BLACK);

    GameState::InGame
}
