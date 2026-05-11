use crate::board::{self, BoardState, Piece, PieceColor, PieceType, to_algebraic};
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

    //en passant
    let dir = if piece.c == PieceColor::Black {-1} else{1};
    match board.en_passant{
        Some((y,x)) => {
            if (piece.pos.0 + dir,piece.pos.1 + 1) == (y,x) || (piece.pos.0 + dir,piece.pos.1 - 1) == (y,x){possible_moves.push((y,x));}
        },
        None => {},
    }
    //captures
    if board.piece_at((piece.pos.0 + dir,piece.pos.1 + 1)).t != PieceType::Empty && board.piece_at((piece.pos.0 + dir,piece.pos.1 + 1)).c == piece.oppose(){
        possible_moves.push((piece.pos.0 + dir,piece.pos.1 + 1));
    }

    if board.piece_at((piece.pos.0 + dir,piece.pos.1 - 1)).t != PieceType::Empty && board.piece_at((piece.pos.0 + dir,piece.pos.1 + 1)).c == piece.oppose(){
        possible_moves.push((piece.pos.0 + dir,piece.pos.1 - 1));
    }

    possible_moves
}

fn get_possible_rook_moves(board: &mut BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    
    fn moves_single_direction(board: &mut BoardState, direction: (i32,i32),squares: i32,piece : &mut Piece) -> Vec<(i32,i32)>{
        let opposing_color = piece.oppose();
        let mut moves: Vec<(i32,i32)> = Vec::new();
            for i in 1..=squares{
                let new_pos = (piece.pos.0 + (i*direction.0),piece.pos.1 + (i*direction.1));
                let p2 = board.piece_at(new_pos);
                match (p2.t,p2.c){
                (PieceType::Empty,PieceColor::Empty) => moves.push(new_pos),
                (PieceType::Empty,_) => panic!("Empty square has a team"),
                (_,c) => {
                    if c == opposing_color{
                        moves.push(new_pos);
                    }
                    break;
                }
            }
            }
            moves

    }
    
    //above
    let direction = (1,0);
    let squares = 7 - piece.pos.0;
    let up = moves_single_direction(board, direction, squares, piece);
    //right
    let direction = (0,1);
    let squares = 7 - piece.pos.1;
    let right = moves_single_direction(board, direction, squares, piece);
    //below
    let direction = (-1,0);
    let squares = piece.pos.0;
    let down = moves_single_direction(board, direction, squares, piece);
    //left
    let direction = (0,-1);
    let squares = piece.pos.1;
    let left = moves_single_direction(board, direction, squares, piece);

    //unify with no duplicates (should not matter)
    let possible_moves: Vec<(i32, i32)> = [
    up,
    down,
    left,
    right,
        ]
    .into_iter()
    .flatten()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();

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
            let new_pos = (position.0 + (squares*d.0),position.1 + (squares*d.1));
            if new_pos.0 >= 0 && new_pos.0 <= 7 && new_pos.1 >= 0 && new_pos.1 <= 7{
                squares = squares + 1;
            }else{break;}
        }
        squares - 1
    }
    
    fn possible_diagonal_single_direction(board: &mut BoardState,piece: &mut Piece,squares_ahead: i32,direction: (i32, i32),opposing_color: PieceColor) -> Vec<(i32,i32)>{
        let mut moves: Vec<(i32,i32)> = Vec::new();
        for i in 1..=squares_ahead{
            let new_pos = ((piece.pos.0 + (i * direction.0)),(piece.pos.1 + (i * direction.1)));
            let p2 = board.piece_at(new_pos);
            match (p2.t,p2.c){
                (PieceType::Empty,PieceColor::Empty) => moves.push(new_pos),
                (PieceType::Empty,_) => panic!("Empty square has a team"),
                (_,c) => {
                    if c == opposing_color{
                        moves.push(new_pos);
                    }
                    break;
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
    direction = (1,-1);
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

    //ADD CASTLING
    //white long castle is ugliest if ever 
    if board.piece_at((0,0)).t == PieceType::Rook &&
    board.piece_at((0,0)).castle_rights &&
    board.piece_at((0,4)).castle_rights &&
    board.piece_at((0,4)).c  == piece.c &&
    board.piece_at((0,1)).t == PieceType::Empty  &&
    board.piece_at((0,2)).t == PieceType::Empty  && 
    board.piece_at((0,3)).t == PieceType::Empty{
        possible_moves.push((0,2));
    }

    //white short castle is ugliest if ever 
    if board.piece_at((0,7)).t == PieceType::Rook &&
    board.piece_at((0,7)).castle_rights &&
    board.piece_at((0,4)).castle_rights &&
    board.piece_at((0,4)).c  == piece.c &&
    board.piece_at((0,5)).t == PieceType::Empty  &&
    board.piece_at((0,6)).t == PieceType::Empty{
        possible_moves.push((0,6));
    }

    //black long castle is ugliest if ever 
    if board.piece_at((7,0)).t == PieceType::Rook &&
    board.piece_at((7,0)).castle_rights &&
    board.piece_at((7,4)).castle_rights &&
    board.piece_at((7,4)).c  == piece.c &&
    board.piece_at((7,1)).t == PieceType::Empty  &&
    board.piece_at((7,2)).t == PieceType::Empty  && 
    board.piece_at((7,3)).t == PieceType::Empty{
        possible_moves.push((7,2));
    }

    //black short castle is ugliest if ever 
    if board.piece_at((7,7)).t == PieceType::Rook &&
    board.piece_at((7,7)).castle_rights &&
    board.piece_at((7,4)).castle_rights &&
    board.piece_at((7,4)).c  == piece.c &&
    board.piece_at((7,5)).t == PieceType::Empty  &&
    board.piece_at((7,6)).t == PieceType::Empty{
        possible_moves.push((7,6));
    }

    possible_moves

}

fn is_capture(board: &mut BoardState, pos: (i32,i32),new_pos: (i32,i32)) -> bool{

    //check for en passant!
    match board.en_passant{
        Some((y,x)) => {
             if board.piece_at(pos).t == PieceType::Pawn && new_pos == (y,x){
                return true
             }
        },
        None => {},
    }

    return board.piece_at((new_pos.0,new_pos.1)).t != PieceType::Empty;
}

fn is_castle(board: &mut BoardState,pos: (i32,i32),new_pos: (i32,i32)) -> &str{
    
    let is_king = board.piece_at(pos).t == PieceType::King;
    let is_horizontal = pos.0 == new_pos.0;
    if !is_king || !is_horizontal {return "none"}
    
    let x_diff = new_pos.1 - pos.1;
    match x_diff{
        2  => return "short",
        -2 => return "long",
        _  => return "none",
    }

}

fn is_promotion(board: &mut BoardState, pos: (i32,i32),new_pos: (i32,i32)) -> bool{
    let p = board.piece_at(pos);
    
    p.t == PieceType::Pawn && ((p.c == PieceColor::Black && new_pos.0 == 0) || (p.c == PieceColor::White && new_pos.0 == 7))
}

fn initial(t: PieceType) -> char{
    match t{
        PieceType::Pawn => return 'P',
        PieceType::Rook => return 'R',
        PieceType::Knight => return 'N',
        PieceType::Bishop => return 'B',
        PieceType::Queen => return 'Q',
        PieceType::King => return 'K',
        PieceType::Empty => panic!("Empty square has no initial"),
    }
}

pub fn doubled_piece_attacks_vertical(board: &mut BoardState,pos: (i32,i32),piece: &mut Piece) -> bool{
    
    let opposing_color = piece.oppose();
    for i in 0..8{
        if i == piece.pos.0 {continue}
        let mut p2 = board.piece_at((i,piece.pos.1));
        if p2.t == PieceType::Empty || p2.c == opposing_color{continue;}
        let moves = get_possible_moves(board, &mut p2);
        // println!("moves are {:?}" ,moves);
        if moves.contains(&pos) && p2.t == piece.t {return true;}
    }
    false
}

pub fn doubled_piece_attacks_horizontal(board: &mut BoardState,pos: (i32,i32),piece: &mut Piece) -> bool{
    
    let opposing_color = piece.oppose();
    for i in 0..8{
        if i == piece.pos.1 {continue}
        let mut p2 = board.piece_at((piece.pos.0,i));
        if p2.t == PieceType::Empty || p2.c == opposing_color{continue;}
        let moves = get_possible_moves(board, &mut p2);
        // println!("moves are {:?} for piece {:?}" ,moves,p2);
        if moves.contains(&pos) && p2.t == piece.t {return true;}
    }
    false
}

pub fn doubled_assymetrical_horses_attack(board: &mut BoardState,pos: (i32,i32),piece: &mut Piece) -> bool{
    let opposing_color = piece.oppose();
    let mut horses: Vec<Piece> = Vec::new();
    let mut potential_knight_count = 0;
    for p in board.playing_pieces.iter_mut(){
        if p.t == PieceType::Knight && p.c != opposing_color{
            // println!("FOUND HORSE: {:?}",p);
            horses.push(*p);
        }
    }

    //second loop since first uses &mut board and i need a second fn to use it here
    for horse in horses.iter_mut(){
        // println!("HORSE:{:?}",horse);
        if get_possible_moves(board, horse).contains(&pos){potential_knight_count = potential_knight_count + 1;}
    }

    potential_knight_count > 1
}


//given a piece and its new location, trusting its right by other code checking for it
pub fn move_to_notation(board: &mut BoardState,piece: &mut Piece,new_pos: (i32,i32))-> String{
    let mut s = String::new();
    let capture = is_capture(board,piece.pos,new_pos);
    let initial = initial(piece.t);
    let new_pos_lettered = to_algebraic(new_pos);
    // println!("new pos is {:?}",new_pos);
    // println!("new pos lettered is {:?}",new_pos_lettered);
    let piece_pos_lettered = to_algebraic(piece.pos);
    //pawn is just letter+num
    //if takes, is letterx letter + number
    //if en passant, the letter + Number is not pawn taken but position moved to
    if initial == 'P'{
        let promotion = is_promotion(board, piece.pos, new_pos);
        //IF PROMOTION ADD =PIECE AT THE END: e8=Q, exd8=Q
        if capture {
            s.push_str(&format!("{}x{}{}",to_algebraic(piece.pos).1,new_pos_lettered.1,new_pos_lettered.0));
        }else{
            s.push_str(&format!("{}{}",new_pos_lettered.1,new_pos_lettered.0));
        }
        if promotion{s.push_str("=Q");}
        return s;
    }

    //all non pawn moves start with piece initial
    s.push(initial);
    
    //if capture 
    
    let vertical = doubled_piece_attacks_vertical(board,new_pos,piece);
    let horizontal = doubled_piece_attacks_horizontal(board, new_pos, piece);
    let assymetrical = doubled_assymetrical_horses_attack(board, new_pos, piece);
    println!("Vertical dup:{} | Horizontal dup:{} | horse assymetrical: {} for piece {:?}",vertical,horizontal,assymetrical,piece.t);
    //if somehow 3 of the same can take and no letter or num is unique then specify by initial letter num x letter num
    if vertical && horizontal{s.push_str(&format!("{}{}",piece_pos_lettered.1,piece_pos_lettered.0));}
    //if 2 same type pieces can take same square they are diff by letter
    else if !vertical && horizontal{s.push_str(&format!("{}",piece_pos_lettered.1));}
    //if on same letter then by num
    else if vertical && !horizontal{s.push_str(&format!("{}",piece_pos_lettered.0));}
    
    else if piece.t == PieceType::Knight && assymetrical{s.push_str(&format!("{}",piece_pos_lettered.1));}

    if capture{
        s.push('x');
    }
    
    s.push_str(&format!("{}{}",new_pos_lettered.1,new_pos_lettered.0));

    let castle_type = is_castle(board, piece.pos, new_pos);
    match castle_type{
        "short" => return "O-O".into(),
        "long" => return "O-O-O".into(),
        "none" => {},
        _ => panic!(),
    }

    s
}

pub fn get_player_possible_moves(board: &mut BoardState,c: PieceColor)-> Vec<String>{    
    let mut moves = Vec::new();
    let mut pieces = board.playing_pieces.clone();
    for piece in pieces.iter_mut(){
        if piece.c != c{continue;} 
        let piece_moves = get_possible_moves(board, piece);
        for piece_move in piece_moves.iter(){
            moves.push(move_to_notation(board, piece, *piece_move));
        }
    }
    moves
}