pub mod ular;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use ular::*;

thread_local! {
    static GAME: RefCell<Game> =
        RefCell::new(Game::new(15, 15, "⬜", "🥓", "🟧", "⬛"));
}

#[wasm_bindgen(js_name = gamePrint)]
pub fn game_print() -> String {
    GAME.with(|game| game.borrow().to_string())
}

#[wasm_bindgen(js_name = gameSpawnFood)]
pub fn game_spawn_food() {
    GAME.with(|game| game.borrow_mut().spawn_food());
}

#[wasm_bindgen(js_name = getSnakeDirection)]
pub fn game_get_snake_direction() -> i8 {
    GAME.with(|game| {
        match game.borrow().get_snake_direction() {
            Direction::Left => 1,
            Direction::Up => 2,
            Direction::Right => 3,
            Direction::Down => 0
        }
    })
}

#[wasm_bindgen(js_name = setSnakeDirection)]
pub fn game_set_snake_direction(code: u8) {
    let code = code % 4;
    let direction = match code {
        1 => Direction::Left,
        2 => Direction::Up,
        3 => Direction::Right,
        _ => Direction::Down
    };
    GAME.with(|game| game.borrow_mut().set_snake_direction(direction));
}

#[wasm_bindgen(js_name = gameTick)]
pub fn game_tick() {
    GAME.with(|game| game.borrow_mut().tick());
}

#[wasm_bindgen(js_name = gameScore)]
pub fn game_score() -> usize {
    GAME.with(|game| game.borrow().get_score())
}

#[wasm_bindgen(js_name = gameIsOver)]
pub fn game_is_over() -> bool {
    GAME.with(|game| game.borrow().is_over())
}

