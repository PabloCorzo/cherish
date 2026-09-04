mod piece_moves;
mod bitboard;
mod render;
mod game;
mod bots;
use dotenv::dotenv;

use std::env;
use crate::{game::{Bot, Game, GameMode}};

//CARGO RUN -- -960 -> chess960 

//CARGO RUN -- -speed -> capture gives you turn again

//CARGO RUN -- -minlog -> log the games' moves

//CARGO RUN -- -h -> human players

//CARGO RUN -- -s -> show all board states as game progresses, for bot games

//ignore if human
//CARGO RUN -- -learn -> bots will call lichess API for opening book creation/appendage,

//CARGO RUN -- -n [num] to play num head to heads, only for bot games
fn main(){
    
    dotenv().ok();
    let args:Vec<String> = env::args().collect();

    
    let n60: bool = args.iter().any(|arg| arg == "-960");
    let speed: bool = args.iter().any(|arg| arg == "-speed");
    
    let minlog: bool = args.iter().any(|arg| arg == "-minlog");
    let human: bool = args.iter().any(|arg| arg == "-h");
    let shown: bool = args.iter().any(|arg| arg == "-s");
    let learn: bool = args.iter().any(|arg| arg == "-learn");


    let n: Option<i32>;
    if let Some(pos) = args.iter().position(|a| a == "-n") {
        if let Some(value) = args.get(pos + 1) {
            n = Some(value.parse().expect("Expected a number after -n"));
        }else { n = None; }
    }else { n = None; }

    
    //default params or set otherwise
    let mut gamemode = GameMode::Std;
    if n60{gamemode = GameMode::N60;}
    else if speed {gamemode = GameMode::Speed;}

    // create game object
    let mut game = match gamemode{
        GameMode::Speed => Game::new_alt(GameMode::Speed,minlog),
        GameMode::N60 => Game::new_alt(GameMode::N60,minlog),
        // GameMode::Std => Game::new_preloaded(b,minlog),
        GameMode::Std => Game::new(minlog),
    };
   

    if human{ game.play_game(); }
    
    // training loop or play head to heads between bots
    else {
        
        println!("START!");
        let result = match n{
            Some(num) => game.play_n_matches(num, Bot::Bard, Bot::Bard,learn,false),
            None => game.play_n_matches(1,Bot::Bard, Bot::Bard,false,shown),
        };
        println!("Result: {:?}",result);
    }
}
