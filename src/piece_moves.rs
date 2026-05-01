use crate::board::{BoardState,Piece,PieceColor,PieceType};
use std::collections::HashSet;

pub fn get_possible_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)>{
    let possible_moves: Vec<(i32,i32)> = match piece.t{
        PieceType::Pawn => get_possible_pawn_moves(board,piece),
        PieceType::Rook => get_possible_rook_moves(board,piece),    
        PieceType::Knight => get_possible_knight_moves(board,piece), 
        PieceType::Bishop => get_possible_bishop_moves(board,piece),
        PieceType::Queen => get_possible_queen_moves(board,piece),
        PieceType::King => get_possible_king_moves(board,piece),
        PieceType::Empty => Vec::new(),
    };
    possible_moves
}

fn get_possible_pawn_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {
    
    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let (first_square,jump_spots) = match piece.c {
        PieceColor::Black => (6,(5,4)),
        PieceColor::White => (1,(2,3)),
        PieceColor::Empty => panic!("Pawn has no color"),
    };
    //has not moved and 2 squares ahead is clear
    if first_square == piece.pos.0 && board.piece_at((jump_spots.1,piece.pos.1)).t == PieceType::Empty{
        possible_moves.push((jump_spots.1,piece.pos.1));
    }
    if board.piece_at((jump_spots.0,piece.pos.1)).t == PieceType::Empty{
        possible_moves.push((jump_spots.0,piece.pos.1))
    }
    possible_moves
}

fn get_possible_rook_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.oppose();

    let direction = 1;
    //werid sub gives amt of squares above rook
    let squares_ahead = 7 - piece.pos.0;
    for i in 1..=squares_ahead{
        //piece above rook
        let pos2 = (piece.pos.0 + (direction * i),piece.pos.1);
        let p2 = board.piece_at(pos2); 
        match (p2.t,p2.c){
            (PieceType::Empty,PieceColor::Empty) => possible_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    possible_moves.push(p2.pos);
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
            (PieceType::Empty,PieceColor::Empty) => possible_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    possible_moves.push(p2.pos);
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
            (PieceType::Empty,PieceColor::Empty) => possible_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    possible_moves.push(p2.pos);
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
            (PieceType::Empty,PieceColor::Empty) => possible_moves.push(p2.pos),
            (PieceType::Empty,_) => panic!("Empty space has a team"),
            (_,c) => {
                if c == opposing_color{
                    possible_moves.push(p2.pos);
                }
                break;
            },
        }
    }
    possible_moves
}

fn get_possible_knight_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
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
            (PieceType::Empty,PieceColor::Empty) => {possible_moves.push(*jump);},
            (PieceType::Empty,_) => panic!("Empty square has a team"),
            (_,c) => {
                if c == opposing_color {possible_moves.push(*jump);}
            },
        }

    }

    possible_moves


}

fn get_possible_bishop_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.oppose();

    fn squares_to_edge(position: (i32,i32),d: (i32,i32)) -> i32{
        let mut squares: i32 = 1;
        loop{
            let new_pos = (position.0 + (1*d.0),position.1 + (1*d.1));
            if new_pos.0 >= 0 && new_pos.0 <= 7 && new_pos.1 >= 0 && new_pos.1 <= 7{
                squares = squares + 1;
            }else{break;}
        }
        squares - 1
    }
    
    fn possible_diagonal_single_direction(board: &mut BoardState,piece: &mut Piece,squares_ahead: i32,direction: (i32, i32),opposing_color: PieceColor) -> Vec<(i32,i32)>{
        let mut moves: Vec<(i32,i32)> = Vec::new();
        for i in 0..squares_ahead{
            let new_pos = ((piece.pos.0 + (i * direction.0)),(piece.pos.0 + (i * direction.0)));
            let p2 = board.piece_at(new_pos);
            match (p2.t,p2.c){
                (PieceType::Empty,PieceColor::Empty) => moves.push(new_pos),
                (PieceType::Empty,_) => panic!("Empty square has a team"),
                (_,c) => {
                    if c == opposing_color{
                        moves.push(new_pos);
                        break;
                    }
                }
            }
        }
        moves
    }
    //up + right
    let mut direction = (1,1);
    let squares_ahead = squares_to_edge(piece.pos, direction);
    let up_right = possible_diagonal_single_direction(board, piece, squares_ahead, direction, opposing_color);
    
    //down + right
    direction = (-1,1);
    let squares_ahead = squares_to_edge(piece.pos, direction);
    let down_right = possible_diagonal_single_direction(board, piece, squares_ahead, direction, opposing_color);

    //down + left
    direction = (-1,-1);
    let squares_ahead = squares_to_edge(piece.pos, direction);
    let down_left = possible_diagonal_single_direction(board, piece, squares_ahead, direction, opposing_color);
    
    //up + left
    direction = (-1,1);
    let squares_ahead = squares_to_edge(piece.pos, direction);
    let up_left = possible_diagonal_single_direction(board, piece, squares_ahead, direction, opposing_color);

    //unify with no duplicates (should not matter)
    let possible_moves: Vec<(i32, i32)> = [
    up_right,
    down_right,
    down_left,
    up_left,
        ]
    .into_iter()
    .flatten()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

    possible_moves
}


fn get_possible_queen_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {
    
    let possible_moves: Vec<(i32, i32)> = [
        get_possible_rook_moves(board, piece),
        get_possible_bishop_moves(board, piece),
        ]
        .into_iter()
    .flatten()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

possible_moves
}

fn get_possible_king_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.oppose();
    for i in -1..2{
        for j in -1..2{
            let new_pos = (piece.pos.0 + i,piece.pos.1 + j);
            //dont check current king position
            if new_pos == piece.pos {continue;}
            //OOB check
            if !(new_pos.0 >= 0 && new_pos.0 <= 7 && new_pos.1 >= 0 && new_pos.1 <= 7) {continue;}
            let p2 = board.piece_at(new_pos);
            match (p2.t,p2.c) {
                (PieceType::Empty,PieceColor::Empty) => possible_moves.push(new_pos),
                (PieceType::Empty,_) => panic!("Empty square has a team"),
                (_, c) => {
                    if c == opposing_color{possible_moves.push(new_pos);}
                },                
            }
        }
    }

    possible_moves

}