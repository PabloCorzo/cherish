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
        .unwrap_or_else(||"std".to_string());

    if play_mode != "std" && play_mode != "aim" {play_mode = String::from("std");}

    let log: bool = args.iter().any(|arg| arg == "-log");

    let mut game = GameManager::new();
    
    game.set_config(&play_mode);
    let _ = game.play_game(log);

}
