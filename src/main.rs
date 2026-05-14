mod board;
mod piece_moves;
mod tests;
mod game_controller;
mod render;
mod input;

use std::env;
use game_controller::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut play_mode: String = args
        .iter()
        .position(|arg| arg == "-mode")
        .and_then(|pos| args.get(pos + 1))
        .cloned()
        .unwrap_or_else(||"tui".to_string());

    if play_mode != "tui" && play_mode != "gui"{play_mode = String::from("tui");}


    let game = GameManager::new();

    game.set_config(&play_mode);

    //IF TUI, DO A RATATUI INIT AND USE TERMINAL
    //IF GUI, DO WHATEVER SLINT NEEDS TO DO TO DRAW ITS GUI

    game.play_game(terminal);

}
