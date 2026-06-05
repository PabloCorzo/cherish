use crate::bitboard::Bitboard;
use crate::piece_moves::player_legal_moves;
use rand::seq::IteratorRandom;

use rand::rngs::{StdRng};
use rand::{SeedableRng, RngExt};

pub struct RandomBot {
    rng: StdRng,
}

impl RandomBot {
    pub fn new() -> Self {
        RandomBot { rng: StdRng::from_rng(&mut rand::rng()) }
    }

    pub fn new_seeded(seed: u64) -> Self {
        RandomBot { rng: StdRng::seed_from_u64(seed) }
    }

    pub fn get_move(&mut self, board: &mut Bitboard) -> (i32,i32,i32){
    let moves = player_legal_moves(board);
    let (from, tos) = moves.iter()
        .filter(|(_,v)| !v.is_empty())
        .choose(&mut self.rng).unwrap();
    let to = tos.iter().choose(&mut self.rng).unwrap();
    let piece = self.rng.random_range(1..=4);
    (*from,*to,piece)
}
}
