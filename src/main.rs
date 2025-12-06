use macroquad::prelude::*;

use crate::{
    game_lost::game_lost_update, game_won::game_won_update, ingame::in_game_update,
    main_menu::main_menu_update,
};

mod game_lost;
mod game_won;
mod ingame;
mod main_menu;

struct InternalState {
    elf: Vec2,
    msg: String,
    clicks: usize,
    misses: usize,
    frames: Vec<Texture2D>,
    frame_index: usize,
    timer: f32,
    game_lost_texture: Texture2D,
    game_won_texture: Texture2D,
}

enum GameState {
    MainMenu,
    InGame,
    GameWon,
    GameLost,
    Exit,
}

async fn run_game() {
    let mut frames = Vec::<Texture2D>::new();

    let size = 110;
    for n in 1..=size {
        let path = format!("./assets/ezgif-split/elf{}.png", n);
        let tex = load_texture(&path).await.unwrap();
        frames.push(tex);
    }

    let mut internal_state = InternalState {
        elf: vec2(screen_width() * 0.5, screen_height() * 0.6),
        msg: String::from("Hit my face if you dare. :)"),
        clicks: 0,
        misses: 0,
        frames,
        frame_index: 0,
        timer: 0.0,
        game_lost_texture: load_texture("./assets/game_lose.png").await.unwrap(),
        game_won_texture: load_texture("./assets/game_win.png").await.unwrap(),
    };

    let mut current_game_state = GameState::MainMenu;

    loop {
        clear_background(WHITE);
        current_game_state = match current_game_state {
            GameState::MainMenu => main_menu_update(&mut internal_state).await,
            GameState::InGame => in_game_update(&mut internal_state),
            GameState::GameWon => game_won_update(&mut internal_state),
            GameState::GameLost => game_lost_update(&mut internal_state),
            GameState::Exit => break,
        };
        next_frame().await
    }
}

#[macroquad::main("Elf on the Shelf")]
async fn main() {
    run_game().await;
}
