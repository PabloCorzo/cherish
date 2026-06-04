mod piece_moves;
mod bitboard;
mod render;
mod game;


use std::env;
use crate::game::{Game,GameMode};
fn main(){
    
    let args:Vec<String> = env::args().collect();

    
    let n60: bool = args.iter().any(|arg| arg == "-960");
    let speed: bool = args.iter().any(|arg| arg == "-s");

    let mut gamemode = GameMode::Std;
    if n60{gamemode = GameMode::N60;}
    else if speed {gamemode = GameMode::Speed;}

    let mut game = match gamemode{
        GameMode::Speed => Game::new_alt(GameMode::Speed),
        GameMode::N60 => Game::new_alt(GameMode::N60),
        GameMode::Std => Game::new(),

    };
    
    game.play_game();
}
