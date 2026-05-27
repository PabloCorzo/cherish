mod bitboard;
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
    let aim: bool = args.iter().any(|arg| arg == "-aim");
    let aim_r: bool = args.iter().any(|arg| arg == "-aim-r");
    
    if aim{play_mode = String::from("aim");}
    if aim_r{play_mode = String::from("aim-r");}
    let mut game = GameManager::new();
    println!("MODE IS {}",play_mode);
    let _result = match play_mode.as_str(){
        "aim" => game.play_aim_game(false),
        "aim-r" => game.play_aim_game(true),
       &_ => game.play_game(log),
    };   
    //let _ = game.play_game(log);

}
