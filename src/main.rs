mod piece_moves;
mod bitboard;
mod render;
mod game;

use crate::bitboard::Bitboard;
use crate::render::render;
fn main(){

    render(&Bitboard::new());
}
