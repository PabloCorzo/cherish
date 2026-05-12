use crate::board::{self, BoardState, Piece, PieceColor, PieceType, to_algebraic};
use crate::piece_moves::*;


//board will have updated both to_move and board. just check if it can move:
//has legal moves
fn was_checkmated(board: &BoardState) -> bool{
    false
}

//king is safe helper fn
//use to_move to check if oppose color is seeing king
fn is_checked(board: &BoardState) -> bool{
    false
}