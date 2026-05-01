use crate::board::{BoardState,Piece,PieceColor,PieceType};
use std::collections::HashSet;

pub fn get_legal_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)>{
    let legal_moves: Vec<(i32,i32)> = match piece.t{
        PieceType::Pawn => get_legal_pawn_moves(board,piece),
        PieceType::Rook => get_legal_rook_moves(board,piece),    
        PieceType::Knight => get_legal_knight_moves(board,piece), 
        PieceType::Bishop => get_legal_bishop_moves(board,piece),
        PieceType::Queen => get_legal_queen_moves(board,piece),
        PieceType::King => get_legal_king_moves(board,piece),
        PieceType::Empty => Vec::new(),
    };
    legal_moves
}

fn get_legal_pawn_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {
    
    let mut legal_moves: Vec<(i32,i32)> = Vec::new();
    let (first_square,jump_spots) = match piece.c {
        PieceColor::Black => (6,(5,4)),
        PieceColor::White => (1,(2,3)),
        PieceColor::Empty => panic!("Pawn has no color"),
    };
    //has not moved and 2 squares ahead is clear
    if first_square == piece.pos.0 && board.piece_at((jump_spots.1,piece.pos.1)).t == PieceType::Empty{
        legal_moves.push((jump_spots.1,piece.pos.1));
    }
    if board.piece_at((jump_spots.0,piece.pos.1)).t == PieceType::Empty{
        legal_moves.push((jump_spots.0,piece.pos.1))
    }
    legal_moves
}

fn get_legal_rook_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut legal_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.oppose();

    let direction = 1;
    //werid sub gives amt of squares above rook
    let squares_ahead = 7 - piece.pos.0;
    for i in 1..=squares_ahead{
        //piece above rook
        let pos2 = (piece.pos.0 + (direction * i),piece.pos.1);
        let p2 = board.piece_at(pos2); 
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => legal_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    legal_moves.push(p2.pos);
                }
                break;
            },
        }
    }
    //squares to the right
    let squares_ahead = 7 - piece.pos.1;
    for i in 1..=squares_ahead{
        //piece above rook
        let pos2 = (piece.pos.0 ,piece.pos.1 + (direction * i));
        let p2 = board.piece_at(pos2); 
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => legal_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    legal_moves.push(p2.pos);
                }
                break;
            },
        }
    }

    let direction = -1;
    //squares below
    let squares_ahead = 7 - (7 + piece.pos.0);
    for i in 1..=squares_ahead{
        //piece above rook
        let pos2 = (piece.pos.0 + (direction * i),piece.pos.1);
        let p2 = board.piece_at(pos2); 
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => legal_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    legal_moves.push(p2.pos);
                }
                break;
            },
        }
    }
    //squares to the left
    let squares_ahead = 7 - (7 + piece.pos.1);
    for i in 1..=squares_ahead{
        //piece above rook
        let pos2 = (piece.pos.0,piece.pos.1 + (direction * i));
        let p2 = board.piece_at(pos2); 
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => legal_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    legal_moves.push(p2.pos);
                }
                break;
            },
        }
    }
    legal_moves
}

fn get_legal_knight_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut legal_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.oppose();
    let mut jumps:Vec<(i32,i32)> = Vec::new();
        jumps.push((piece.pos.0 + 2,piece.pos.1 - 1));
        jumps.push((piece.pos.0 + 2,piece.pos.1 + 1));
        jumps.push((piece.pos.0 + 1,piece.pos.1 - 2));
        jumps.push((piece.pos.0 + 1,piece.pos.1 + 2));
        jumps.push((piece.pos.0 - 2,piece.pos.1 - 1));
        jumps.push((piece.pos.0 - 2,piece.pos.1 + 1));
        jumps.push((piece.pos.0 - 1,piece.pos.1 - 2));
        jumps.push((piece.pos.0 - 1,piece.pos.1 + 2));
    for jump in jumps.iter(){
        //Jump OOB check
        if !(0 <= jump.0 && jump.0 <= 7 && 0 <= jump.1 && jump.1 <= 7) {continue;}

        let p2 = board.piece_at(*jump);
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => {legal_moves.push(*jump);},
            (PieceType::Empty,_) => panic!("Empty square has a team"),
            (_,c) => {
                if c == opposing_color {legal_moves.push(*jump);}
            },
        }

    }

    legal_moves


}

fn get_legal_bishop_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut legal_moves: Vec<(i32,i32)> = Vec::new();

    legal_moves

}

fn get_legal_king_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut legal_moves: Vec<(i32,i32)> = Vec::new();
    
    legal_moves

}

fn get_legal_queen_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let legal_moves: Vec<(i32, i32)> = [
    get_legal_rook_moves(board, piece),
    get_legal_bishop_moves(board, piece),
        ]
    .into_iter()
    .flatten()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

    legal_moves
}