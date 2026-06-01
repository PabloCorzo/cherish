mod piece_moves;
mod bitboard;
mod render;
mod game;

use crate::render::render;
use crate::game::Game;

fn main(){

    let mut game = Game::new();
    game.play_game();
}
