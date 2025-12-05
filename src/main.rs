use macroquad::{prelude::*, rand::gen_range};

#[macroquad::main("Elf on the Shelf")]
async fn main() {
    let mut elf = vec2(screen_width() * 0.5, screen_height() * 0.6);
    let mut msg = String::from("Hit my face if you dare.");
    let mut clicks = 0;
    let mut misses = 0;

    let mut frames = Vec::<Texture2D>::new();

    let size = 110;
    for n in 1..=size {
        let path = format!("./assets/ezgif-split/elf{}.png", n);
        let tex = load_texture(&path).await.unwrap();
        frames.push(tex);
    }
    let mut frame_index = 0;
    let mut timer = 0.0;

    loop {
        clear_background(WHITE);

        // shelf
        let shelf_height = 40.0;
        let shelf_y = screen_height() * 0.75;
        draw_rectangle(0.0, shelf_y, screen_width(), shelf_height, BROWN);

        // elf face
        let elf_r = 22.0;

        timer += get_frame_time();
        if timer > 0.02 {
            frame_index = (frame_index + 1) % frames.len();
            timer = 0.00;
        }

        draw_texture(&frames[frame_index], elf.x - 250.0, elf.y - 95.0, WHITE);

        // click logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let distance_from_elf = vec2(mouse_x, mouse_y).distance(elf);
            if distance_from_elf <= elf_r + 8.0 {
                // hit
                clicks += 1;
            } else {
                misses += 1;
            }
            msg = format!("You hit my face: {clicks}, Missed: {misses}");
            elf.x = gen_range(30.0, screen_width() - 30.0);
            elf.y = gen_range(50.0, shelf_y - elf_r - 10.0);
        }

        draw_text(&msg, 20.0, 30.0, 28.0, BLACK);

        next_frame().await
    }
}
