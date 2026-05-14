mod board;
mod piece_moves;
mod tests;
mod game_controller;
mod render;
mod input;

use std::env;
use std::io::stdout;
use game_controller::*;
use ratatui::{Terminal, backend::CrosstermBackend};
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut play_mode: String = args
        .iter()
        .position(|arg| arg == "-mode")
        .and_then(|pos| args.get(pos + 1))
        .cloned()
        .unwrap_or_else(||"tui".to_string());

    if play_mode != "tui" && play_mode != "gui"{play_mode = String::from("tui");}


    let mut game = GameManager::new();

    game.set_config(&play_mode);

    if play_mode == "tui" {
        enable_raw_mode().unwrap();
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        let _ = game.play_game(Some(&mut terminal));
        disable_raw_mode().unwrap();
    } else {
        let _ = game.play_game(None);
    }

}
