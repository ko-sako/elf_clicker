use macroquad::{
    audio::{Sound, load_sound},
    prelude::*,
};

use crate::{
    game_lost::game_lost_update, game_won::game_won_update, ingame::in_game_update,
    ingame_easy::in_game_easy_update, ingame_hard::in_game_hard_update,
    main_menu::main_menu_update,
};

mod game_lost;
mod game_won;
mod ingame;
mod ingame_easy;
mod ingame_hard;
mod main_menu;

struct InternalState {
    elf: Vec2,
    elf2: Vec2,
    msg: String,
    clicks: usize,
    misses: usize,
    clicks2: usize,
    frames: Vec<Texture2D>,
    frames2: Vec<Texture2D>,
    frame_index: usize,
    timer: f32,
    ingame_texture: Texture2D,
    game_lost_texture: Texture2D,
    game_won_texture: Texture2D,
    mouse_texture: Texture2D,
    ingame_bgm: Sound,
    ingame_hard_bgm: Sound,
    game_lost_bgm: Sound,
    game_won_bgm: Sound,
    hit_sound: Sound,
    hit2_sound: Sound,
    gun_sound: Sound,
}

enum GameState {
    MainMenu,
    InGame,
    InGameHard,
    InGameEasy,
    GameWon,
    GameLost,
    Exit,
}

async fn run_game() {
    let mut frames = Vec::<Texture2D>::new();
    let mut frames2 = Vec::<Texture2D>::new();

    let size = 110;
    for n in 1..=size {
        let path = format!("./assets/ezgif-split/elf{}.png", n);
        let tex = load_texture(&path).await.unwrap();
        frames.push(tex);
        let path2 = format!("./assets/ezgif-split_2/elf{}.png", n);
        let tex2 = load_texture(&path2).await.unwrap();
        frames2.push(tex2);
    }

    let mut internal_state = InternalState {
        elf: vec2(screen_width() * 0.5, screen_height() * 0.6),
        elf2: vec2(screen_width() * 0.6, screen_height() * 0.5),
        msg: String::from("Hit my face if you dare. :)"),
        clicks: 0,
        misses: 0,
        frames,
        frames2,
        frame_index: 0,
        timer: 0.0,
        game_lost_texture: load_texture("./assets/game_lose.png").await.unwrap(),
        game_won_texture: load_texture("./assets/game_win.png").await.unwrap(),
        ingame_texture: load_texture("./assets/ingame_bg.png").await.unwrap(),
        mouse_texture: load_texture("./assets/mouse.png").await.unwrap(),
        clicks2: 0,
        ingame_bgm: load_sound("assets/sound/ingame_bgm.ogg").await.unwrap(),
        ingame_hard_bgm: load_sound("assets/sound/ingame_hard_bgm.ogg")
            .await
            .unwrap(),
        game_lost_bgm: load_sound("assets/sound/game_lose.ogg").await.unwrap(),
        game_won_bgm: load_sound("assets/sound/game_won_bgm.ogg").await.unwrap(),
        hit_sound: load_sound("assets/sound/elf_voice.ogg").await.unwrap(),
        hit2_sound: load_sound("assets/sound/elf_voice_1.ogg").await.unwrap(),
        gun_sound: load_sound("assets/sound/hit.ogg").await.unwrap(),
    };

    let mut current_game_state = GameState::MainMenu;

    loop {
        clear_background(WHITE);
        current_game_state = match current_game_state {
            GameState::MainMenu => main_menu_update(&mut internal_state).await,
            GameState::InGame => in_game_update(&mut internal_state).await,
            GameState::GameWon => game_won_update(&mut internal_state).await,
            GameState::GameLost => game_lost_update(&mut internal_state).await,
            GameState::Exit => break,
            GameState::InGameHard => in_game_hard_update(&mut internal_state).await,
            GameState::InGameEasy => in_game_easy_update(&mut internal_state).await,
        };
        next_frame().await
    }
}

#[macroquad::main("Snitch on the Shelf")]
async fn main() {
    run_game().await;
}
