mod piece_moves;
mod bitboard;
mod render;
mod game;
mod bots;
use dotenv::dotenv;

use std::env;
use crate::{bitboard::Bitboard, game::{Bot, Game, GameMode}};
fn main(){
    
    dotenv().ok();
    let args:Vec<String> = env::args().collect();

    
    let n60: bool = args.iter().any(|arg| arg == "-960");
    let speed: bool = args.iter().any(|arg| arg == "-speed");
    let minlog: bool = args.iter().any(|arg| arg == "-minlog");
    let human: bool = args.iter().any(|arg| arg == "-h");
    let _shown: bool = args.iter().any(|arg| arg == "-s");
    let debug: bool = args.iter().any(|arg| arg == "-debug");

    let mut n: Option<i32>;
    if let Some(pos) = args.iter().position(|a| a == "-n") {
        if let Some(value) = args.get(pos + 1) {
            n = Some(value.parse().expect("Expected a number after -n"));
        }else { n = None; }
    }else { n = None; }



    let mut gamemode = GameMode::Std;
    if n60{gamemode = GameMode::N60;}
    else if speed {gamemode = GameMode::Speed;}

    let b = match debug{
        false => Bitboard::new(), 
        true => Bitboard::new_from_fen("8/8/3B4/4K1k1/8/8/8/3R4 w - - 0 1"),
    };
    let mut game = match gamemode{
        GameMode::Speed => Game::new_alt(GameMode::Speed,minlog),
        GameMode::N60 => Game::new_alt(GameMode::N60,minlog),
        GameMode::Std => Game::new_preloaded(b,minlog),
        //GameMode::Std => Game::new(minlog),
    };
    
    if human{ game.play_game(); }

    else {

        println!("START!");
        let result = match n{
            Some(num) => game.play_n_matches(num, Bot::Bard, Bot::Bard),
            None => game.play_head_to_head(Bot::Bard, Bot::Bard),
        };
        println!("Result: {:?}",result);
    }
}
