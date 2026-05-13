use crate::board::{self, BoardState, Piece, PieceColor, PieceType, to_algebraic};
use crate::piece_moves::*;


//board will have updated both to_move and board. just check if it can move:
//has legal moves
//check playing color or color to play
fn is_checkmated(board: &BoardState) -> bool{
    
    //Has moves
    let has_moves = get_player_legal_moves(board, board.to_move).is_empty();

    //Is in check
    let in_check = is_checked(board,board.to_move);

    !has_moves && in_check
}

fn is_stalemate(board: &BoardState) -> bool{
    
    //Has moves
    let has_moves = get_player_legal_moves(board, board.to_move).is_empty();
    
    //Is in check
    let in_check = is_checked(board,board.to_move);

    !has_moves && !in_check

    //======================TODO======================//
    //================================================//
    //================================================//
    //              x moves w no captures             //
    //              board repeats 3 times             //
}

//king is safe helper fn
//use to_move to check if oppose color is seeing king
fn is_checked(board: &BoardState, c: PieceColor) -> bool{

    let king_pos = board.playing_pieces
    .iter()
    .find(|p| p.c == c && p.t == PieceType::King)
    .expect(&format!("Player {:?} has no king in piece vector", c))
    .pos;

    get_player_legal_moves(board, c.oppose())
    .values()
    .flatten()
    .any(|p| *p == king_pos)

}


//assumes given move is valid. use wisely or it will break boardstate!
fn player_move(board: &mut BoardState, piece: &mut Piece, pos: (i32,i32)) -> &'static str{
    
    if piece.c == PieceColor::Empty || piece.t == PieceType::Empty{panic!("Tried to move either a colorless piece or empty square")} 

    //en passant in state for move checks next turn
    let allows_en_passant = piece.t == PieceType::Pawn && 
    ((piece.pos.0 - pos.0 == 2) || 
    (piece.pos.0 - pos.0 == -2)); 

    match allows_en_passant {
        true => board.en_passant = Some(pos),
        false => board.en_passant = None,
    }


    //castling rights revoke
    piece.castle_rights = false;

    //is capture? if so, remove piece from vector
    let dest_piece = board.piece_at(pos);
    match dest_piece.c == piece.c{
        true => {panic!("Cannot move to place occupied by same color piece.")},
        false => {
            board.playing_pieces = board.playing_pieces
            .iter()
            .filter(|p| p.pos != pos)
            .cloned()
            .collect();
        },
    }

    // now that piece might have been removed, you should move the piece!
    //this fn handles:
    //leaving original spot empty -> placing piece in destination and updating piece position
    //rest stays here due to separation of concerns, game logic goes here
    board.move_piece(piece, pos);


    //switch turn 
    board.to_move = board.to_move.oppose();

    // determine result
    let checkmate = is_checkmated(board);
    let stalemate = is_stalemate(board);
    match (checkmate,stalemate) {
        (true,true) => panic!("Cant be stalemated and checkmated at once."),
        (true,false) => "checkmate",
        (false,true) => "stalemate",
        (false,false) => "none",
    }
}


struct game_controller{
    input: fn() -> &'static str,
    render: fn(&BoardState),
}
