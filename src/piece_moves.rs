use crate::board::{self, BoardState, Piece, PieceColor, PieceType, to_algebraic};
use std::{collections::{HashMap, HashSet}, ops::Index};

pub fn get_possible_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)>{
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

fn get_possible_pawn_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {
    
    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let (first_square,jump_spots) = match piece.c {
        PieceColor::Black => (6,(5,4)),
        PieceColor::White => (1,(2,3)),
        PieceColor::Empty => panic!("Pawn has no color"),
    };
    //has not moved and 2 squares ahead is clear
    if first_square == piece.pos.0 && board.piece_at((jump_spots.0,piece.pos.1)).t == PieceType::Empty && board.piece_at((jump_spots.1,piece.pos.1)).t == PieceType::Empty{
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
    if board.piece_at((piece.pos.0 + dir,piece.pos.1 + 1)).t != PieceType::Empty && board.piece_at((piece.pos.0 + dir,piece.pos.1 + 1)).c == piece.c.oppose(){
        possible_moves.push((piece.pos.0 + dir,piece.pos.1 + 1));
    }

    if board.piece_at((piece.pos.0 + dir,piece.pos.1 - 1)).t != PieceType::Empty && board.piece_at((piece.pos.0 + dir,piece.pos.1 - 1)).c == piece.c.oppose(){
        possible_moves.push((piece.pos.0 + dir,piece.pos.1 - 1));
    }

    possible_moves
}

fn get_possible_rook_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    
    fn moves_single_direction(board: &BoardState, direction: (i32,i32),squares: i32,piece : &mut Piece) -> Vec<(i32,i32)>{
        let opposing_color = piece.c.oppose();
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

fn get_possible_knight_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.c.oppose();
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

fn get_possible_bishop_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.c.oppose();

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
    
    fn possible_diagonal_single_direction(board: &BoardState,piece: &mut Piece,squares_ahead: i32,direction: (i32, i32),opposing_color: PieceColor) -> Vec<(i32,i32)>{
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


fn get_possible_queen_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {
    
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

fn get_possible_king_moves(board: &BoardState,piece : &mut Piece) -> Vec<(i32,i32)> {

    let mut possible_moves: Vec<(i32,i32)> = Vec::new();
    let opposing_color = piece.c.oppose();
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

    let white_long_clear  = [(0,1),(0,2),(0,3)];
    let white_short_clear = [(0,5),(0,6)];
    let black_long_clear  = [(7,1),(7,2),(7,3)];
    let black_short_clear = [(7,5),(7,6)];

    // squares the king passes through (incl. start & destination) — must not be attacked
    let white_long_safe   = [(0,2),(0,3),(0,4)];
    let white_short_safe  = [(0,4),(0,5),(0,6)];
    let black_long_safe   = [(7,2),(7,3),(7,4)];
    let black_short_safe  = [(7,4),(7,5),(7,6)];

    // build attacked set; handle enemy king with raw steps to avoid recursion
    let enemy_pieces: Vec<Piece> = board.playing_pieces.iter()
        .filter(|p| p.c == opposing_color)
        .copied()
        .collect();
    let mut attacked: HashSet<(i32,i32)> = HashSet::new();
    for mut p in enemy_pieces {
        if p.t == PieceType::King {
            for i in -1i32..=1 { for j in -1i32..=1 {
                let sq = (p.pos.0 + i, p.pos.1 + j);
                if sq != p.pos && sq.0 >= 0 && sq.0 <= 7 && sq.1 >= 0 && sq.1 <= 7 {
                    attacked.insert(sq);
                }
            }}
        } else {
            attacked.extend(get_possible_moves(board, &mut p));
        }
    }

    if piece.c == PieceColor::White && piece.castle_rights {
        if board.piece_at((0,0)).t == PieceType::Rook && board.piece_at((0,0)).castle_rights
        && white_long_clear.iter().all(|&sq| board.piece_at(sq).t == PieceType::Empty)
        && white_long_safe.iter().all(|sq| !attacked.contains(sq)) {
            possible_moves.push((0,2));
        }
        if board.piece_at((0,7)).t == PieceType::Rook && board.piece_at((0,7)).castle_rights
        && white_short_clear.iter().all(|&sq| board.piece_at(sq).t == PieceType::Empty)
        && white_short_safe.iter().all(|sq| !attacked.contains(sq)) {
            possible_moves.push((0,6));
        }
    }

    if piece.c == PieceColor::Black && piece.castle_rights {
        if board.piece_at((7,0)).t == PieceType::Rook && board.piece_at((7,0)).castle_rights
        && black_long_clear.iter().all(|&sq| board.piece_at(sq).t == PieceType::Empty)
        && black_long_safe.iter().all(|sq| !attacked.contains(sq)) {
            possible_moves.push((7,2));
        }
        if board.piece_at((7,7)).t == PieceType::Rook && board.piece_at((7,7)).castle_rights
        && black_short_clear.iter().all(|&sq| board.piece_at(sq).t == PieceType::Empty)
        && black_short_safe.iter().all(|sq| !attacked.contains(sq)) {
            possible_moves.push((7,6));
        }
    }

    possible_moves

}

fn is_capture(board: &BoardState, pos: (i32,i32),new_pos: (i32,i32)) -> bool{

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

fn is_castle(board: &BoardState,pos: (i32,i32),new_pos: (i32,i32)) -> &str{
    
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

fn is_promotion(board: &BoardState, pos: (i32,i32),new_pos: (i32,i32)) -> bool{
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

pub fn doubled_piece_attacks_vertical(board: &BoardState,pos: (i32,i32),piece: &mut Piece) -> bool{
    
    let opposing_color = piece.c.oppose();
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

pub fn doubled_piece_attacks_horizontal(board: &BoardState,pos: (i32,i32),piece: &mut Piece) -> bool{
    
    let opposing_color = piece.c.oppose();
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
    let opposing_color = piece.c.oppose();
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


pub fn get_player_possible_moves(board: &BoardState,c: PieceColor) -> HashMap<(i32,i32),Vec<(i32,i32)>>{    
    let mut moves = HashMap::new();
    let mut pieces = board.playing_pieces.clone();
    for piece in pieces.iter_mut(){
        if piece.c != c{continue;} 
        moves.insert(piece.pos,Vec::new());
        let piece_moves = get_possible_moves(board, piece);
        let v = moves.get_mut(&piece.pos).unwrap();
        for piece_move in piece_moves.into_iter(){
            let p = *&piece_move.clone();
            v.push(p);
        }
    }
    moves
}

fn is_pinned(board: &BoardState,piece: &mut Piece,new_pos: (i32,i32)) -> bool{

    let mut board2 = BoardState::new();
    board2.board = board.board.clone();
    board2.board[new_pos.0 as usize][new_pos.1 as usize] = piece.clone();
    board2.board[piece.pos.0 as usize][piece.pos.1 as usize] = Piece {
        t: PieceType::Empty,
        c: PieceColor::Empty,
        pos: (piece.pos.0,piece.pos.1),
        castle_rights: false
    };

    // sync playing_pieces with the simulated move: remove captured piece, update moved piece pos
    board2.playing_pieces = board.playing_pieces
        .iter()
        .filter(|p| p.pos != new_pos)
        .cloned()
        .collect();
    if let Some(p) = board2.playing_pieces.iter_mut().find(|p| p.pos == piece.pos) {
        p.pos = new_pos;
    }

    let king_pos = board.playing_pieces
    .iter()
    .find(|p|p.t == PieceType::King && p.c == piece.c)
    .expect("King is not in playing pieces? how bro").pos;

    let enemy_c = piece.c.oppose();
    let in_check = get_player_possible_moves(&board2, enemy_c)
    .values()
    .flatten()
    .any(|&pos| pos == king_pos);
    in_check
}

pub fn get_player_legal_moves(board: &BoardState, c: PieceColor) -> HashMap<(i32,i32), Vec<(i32,i32)>> {
    
    let possible_moves: HashMap<(i32, i32), Vec<(i32, i32)>> = get_player_possible_moves(board, c);

    possible_moves
        .into_iter()
        .filter_map(|(piece_pos, moves)| {
            // Clone the piece out before any borrow of board inside the closure
            let mut piece = board.playing_pieces
                .iter()
                .find(|p| p.pos == piece_pos)
                .unwrap()
                .clone();

            let legal: Vec<(i32, i32)> = moves
                .into_iter()
                .filter(|&new_pos| !is_pinned(&board, &mut piece, new_pos))
                .collect();

            if legal.is_empty() {
                None
            } else {
                Some((piece_pos, legal))
            }
        })
        .collect()
}
