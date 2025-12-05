use macroquad::{prelude::*, rand::gen_range};

#[macroquad::main("Elf on the Shelf")]
async fn main() {
    let mut elf = vec2(screen_width() * 0.5, screen_height() * 0.6);
    let mut msg = String::from("Hit my face if you dare.");
    let mut clicks = 0;
    let mut misses = 0;

    let elf_tex = load_texture("./assets/elf.png").await.unwrap();
    // elf_tex.set_filter(FilterMode::Nearest);

    loop {
        clear_background(WHITE);

        // shelf
        let shelf_height = 40.0;
        let shelf_y = screen_height() * 0.75;
        draw_rectangle(0.0, shelf_y, screen_width(), shelf_height, BROWN);

        // elf face
        let elf_r = 22.0;
        draw_circle(elf.x, elf.y, elf_r, BEIGE);
        // elf red hat
        draw_triangle(
            vec2(elf.x, elf.y - elf_r - 6.0),
            vec2(elf.x - 14.0, elf.y - elf_r + 8.0),
            vec2(elf.x + 14.0, elf.y - elf_r + 8.0),
            RED,
        );
        // elf eyes
        draw_circle(elf.x - 7.0, elf.y - 4.0, 3.0, BLACK);
        draw_circle(elf.x + 7.0, elf.y - 4.0, 3.0, BLACK);

        draw_texture(&elf_tex, elf.x - 80.0, elf.y - 80.0, WHITE);

        // click logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let distance_from_elf = vec2(mouse_x, mouse_y).distance(elf);
            if distance_from_elf <= elf_r + 8.0 {
                // hit
                clicks += 1;

                elf.x = gen_range(30.0, screen_width() - 30.0);
                elf.y = gen_range(50.0, shelf_y - elf_r - 10.0);
            } else {
                misses += 1;
            }
            msg = format!("You hit my face: {clicks}, Missed: {misses}");
        }

        draw_text(&msg, 20.0, 30.0, 28.0, BLACK);

        next_frame().await
    }
}
