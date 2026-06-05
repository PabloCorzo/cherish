mod piece_moves;
mod bitboard;
mod render;
mod game;
mod bots;

use std::env;
use crate::game::{Game,GameMode,Bot};
fn main(){
    
    let args:Vec<String> = env::args().collect();

    
    let n60: bool = args.iter().any(|arg| arg == "-960");
    let speed: bool = args.iter().any(|arg| arg == "-s");
    let minlog: bool = args.iter().any(|arg| arg == "-minlog");
    let human: bool = args.iter().any(|arg| arg == "-h");
    let shown: bool = args.iter().any(|arg| arg == "-s");


    let mut gamemode = GameMode::Std;
    if n60{gamemode = GameMode::N60;}
    else if speed {gamemode = GameMode::Speed;}

    let mut game = match gamemode{
        GameMode::Speed => Game::new_alt(GameMode::Speed,minlog),
        GameMode::N60 => Game::new_alt(GameMode::N60,minlog),
        GameMode::Std => Game::new(minlog),
    };
    
    if human{ game.play_game(); }

    else {
        println!("START!");
        game.bot_game(Bot::Random,Bot::Random,shown); 
    }
}
