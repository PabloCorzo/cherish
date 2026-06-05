use crate::{bitboard::Bitboard, render::render};
use std::{collections::HashMap};

fn pawn_possible_moves(board: &Bitboard,piece: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();
    let c = board.to_move;
    let (one_step,two_step): (i32,i32) = match c{
        1  => (piece + 8,piece + 16),
        -1 => (piece - 8,piece - 16),
        _ => panic!("Pawn has invalid color"),
    };
    let (dl,dr): (i32,i32) = match c {
        1  => (piece + 7,piece + 9),
        -1 => (piece - 9,piece - 7),
        _ => panic!("Pawn has invalid color"),
    };

    // Do abs of color_of - c, result will be:
    //     0 for ally
    //     1 for empty
    //     2 for enemy

    let can_jump: bool = match c{
        1  => piece >= 8 && piece <= 15,
        -1 => piece >= 48 && piece <= 55,
        _ => panic!("Invalid pawn color"),
    };
    let team = (board.color_of(one_step) - c).abs();
    match team{
        0 => {},
        1 => moves.push(one_step),
        2 => {},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let team = (board.color_of(two_step) - c).abs();
    match team{
        0 => {},
        1 => if can_jump && !moves.is_empty(){moves.push(two_step);},
        2 => {},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let file = piece % 8;
    let team = (board.color_of(dl) - c).abs();
    match team{
        0 => {},
        1 => {},
        2 => if file > 0 {moves.push(dl)},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let team = (board.color_of(dr) - c).abs();
    match team{
        0 => {},
        1 => {},
        2 => if file < 7{moves.push(dr)},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    //en passant check
    if board.en_passant == -1 {return moves;}

    if board.en_passant == dr {moves.push(dr);}
    if board.en_passant == dl {moves.push(dl);}

    moves
}
// Knight move offsets, N = piece location (e.g. e4):
//
//   a    b    c    d    e    f    g    h
//   .    .    .   +15   .   +17   .    .    <- rank 6
//   .    .   +6    .    .    .   +10   .    <- rank 5
//   .    .    .    .    N    .    .    .    <- rank 4
//   .    .   -10   .    .    .   -6    .    <- rank 3
//   .    .    .   -17   .   -15   .    .    <- rank 2

fn knight_possible_moves(board: &Bitboard,piece: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();
    let c = board.to_move;
    let jumps: [i32;8] = [piece + 6,piece - 10,piece + 15,piece - 17,piece + 10, piece - 6, piece + 17,piece - 15];
    let file = piece % 8;
    let left_free = file;
    let right_free = 7 - file;
    let top_free = 7 - (piece / 8);
    let bot_free = piece / 8;

    //2 left, 1 up/down
    if left_free > 1 {
        if top_free > 0{
            let team = (board.color_of(jumps[0]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[0]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
        if bot_free > 0{
            let team = (board.color_of(jumps[1]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[1]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
    }
    //1 left, 2 up/down
    if left_free > 0{
        if top_free > 1{
            let team = (board.color_of(jumps[2]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[2]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
        if bot_free > 1{
            let team = (board.color_of(jumps[3]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[3]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
    }
    //2 right, 1 up/down
    if right_free > 1{
        if top_free > 0{
            let team = (board.color_of(jumps[4]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[4]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
        if bot_free > 0{
            let team = (board.color_of(jumps[5]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[5]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
    }
    //1 right, 2 up/down
    if right_free > 0{
        if top_free > 1{
            let team = (board.color_of(jumps[6]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[6]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
        if bot_free > 1{
            let team = (board.color_of(jumps[7]) - c).abs();
            match team {
                0 => {},  // ally, skip
                1 | 2 => moves.push(jumps[7]),  // empty or enemy
                _ => panic!("Invalid team"),
            }
        }
    }
    moves
} 
 
  
fn rook_possible_moves(board: &Bitboard, piece: i32) -> Vec<i32>{
    let c = board.to_move;
    fn move_until_blocked(board: &Bitboard,dir: (i32,i32),piece: i32,c: i32) -> Vec<i32>{
        let mut squares: Vec<i32> = Vec::new();
        let mut pos = piece;
        let rank = piece / 8;
        let file = piece % 8;
        //dir is (y,x), where y and x are -1, 0 or 1. One has to be 0 and the other either -1 or 1
        pos = pos + 8 * dir.0;
        pos = pos + dir.1;
        while pos <= 63 && pos >= 0{
            if dir.0 != 0 && (pos % 8) != file{break;} 
            if dir.1 != 0 && (pos / 8) != rank{break;} 
            let team = (board.color_of(pos) - c).abs();
            match team{
                0 => break,
                1 => squares.push(pos),
                2 => {
                    squares.push(pos);
                    break;
                },
                _ => panic!("Team identification via color of subtract is wrong."),
            }
            pos = pos + 8 * dir.0;
            pos = pos + dir.1;
        }
        squares
    }
    

    let up_moves = move_until_blocked(board, (1,0), piece, c);
    let down_moves = move_until_blocked(board, (-1,0), piece, c);
    let left_moves = move_until_blocked(board, (0,-1), piece, c);
    let right_moves = move_until_blocked(board, (0,1), piece, c);
    
    let moves = [up_moves,down_moves,left_moves,right_moves]
        .into_iter()
        .flatten()
        .collect();
    moves
}

fn bishop_possible_moves(board: &Bitboard,piece: i32) -> Vec<i32>{
    let c = board.to_move;
    fn move_until_blocked(board: &Bitboard,dir: (i32,i32),piece: i32,c: i32) -> Vec<i32>{
        let mut squares: Vec<i32> = Vec::new();
        
        let mut pos = piece;
        let mut rank = piece / 8;
        let mut file = piece % 8;
        pos = pos + 8 * dir.0;
        pos = pos + dir.1;
        while pos <= 63 && pos >= 0{
            let y_diff = ((pos % 8) - file).abs();  
            let x_diff = ((pos / 8) - rank).abs();
            rank = pos / 8;
            file = pos % 8;
            if y_diff > 1 || x_diff > 1{break;} 
            let team = (board.color_of(pos) - c).abs();
            match team{
                0 => break,
                1 => squares.push(pos),
                2 => {
                    squares.push(pos);
                    break;
                },
                _ => panic!("Team identification via color of subtract is wrong."),
            } 
            pos = pos + 8 * dir.0;
            pos = pos + dir.1;
        } 
        squares
    } 
     
 
    let up_right = move_until_blocked(board, (1,1), piece, c);
    let down_right = move_until_blocked(board, (-1,1), piece, c);
    let up_left = move_until_blocked(board, (1,-1), piece, c);
    let down_left = move_until_blocked(board, (-1,-1), piece, c);
     
    let moves = [up_left,up_right,down_left,down_right]
        .into_iter()
        .flatten()
        .collect();
    moves 
} 
 
fn queen_possible_moves(board: &Bitboard,piece: i32) -> Vec<i32>{
    let c = board.to_move;
    let bm = bishop_possible_moves(board, piece);
    let rm = rook_possible_moves(board, piece);
    [rm,bm].into_iter().flatten().collect()
}

fn king_possible_moves(board: &Bitboard,piece: i32) -> Vec<i32>{
    let c = board.to_move;
    let mut moves: Vec<i32> =  Vec::new();
    let mut squares: [i32;8] = [piece + 7,piece + 8,piece + 9, piece -1,piece + 1, piece -7, piece - 8, piece - 9];
    
    //check if bit offsets are right for edge positions
    let file = piece % 8;
    let left_free = file;
    let right_free = 7 - file;
    let top_free = 7 - (piece / 8);
    let bot_free = piece / 8;
    
    if left_free == 0{
        squares[0] = 64;
        squares[3] = 64;
        squares[5] = 64;
    }

    if right_free == 0{
        squares[2] = 64;
        squares[4] = 64;
        squares[7] = 64;
    }
    if top_free == 0{
        squares[1] = 64;
    }
    if bot_free == 0{
        squares[6] = 64;
    }

    for square in squares.into_iter(){
        if square < 0 || square > 63 {continue;}
        let team = (board.color_of(square) - c).abs();
        match team{
            0 => continue,
            1 => moves.push(square),
            2 => {
                moves.push(square);
                break;
            },
            _ => panic!("Team identification via color of subtract is wrong."),
        }
    }

    //look for rooks to the left and right whose castle rights are up
    //horizontal check by iterating rooks and checking for rank?
    let mut rooks = match c {
        -1 => board.br,
        1 => board.wr,
        _ => panic!("Invalid color."),        
    };
    let mut possible_rooks:Vec<i32> = Vec::new();
    let king_rank = piece / 8;
    let mut rank;

    if !board.castle_rights.contains(&piece) {return moves;} 
    while rooks != 0{
        let sq = rooks.trailing_zeros() as i32; //gives LSB
        rooks &= rooks - 1;
        rank = sq / 8;
        if rank == king_rank && board.castle_rights.contains(&sq){possible_rooks.push(sq);}

    }

    //for each i32 rook pos, iterate from king to rook and check all squares are empty
    // Castling therefore requires horizontal alignment and no moves beforehand for rook and king
    // meaning if you have 3 rooks at the start, one of which if next to the king, you can castle w it
    for rook in possible_rooks.into_iter(){
        let from = std::cmp::min(piece,rook) + 1;   
        let to = std::cmp::max(piece,rook);   
        let mut pieces: Vec<i32> = Vec::new();
        for square in from..to{
        pieces.push((board.color_of(square) - c).abs());
        }
        if pieces.iter().all(|&t| t == 1){
            moves.push(rook);
        }
    }

    moves
}
fn player_possible_moves(board: &Bitboard,include_king: bool) -> HashMap<i32,Vec<i32>>{
    let c = board.to_move; 
    let(mut pawns,mut knights,mut bishops,mut rooks,mut queens,mut king) = match c{
        1  => (board.wp,board.wn,board.wb,board.wr,board.wq,board.wk),
        -1 => (board.bp,board.bn,board.bb,board.br,board.bq,board.bk),
        _ => panic!("Color is invalid for player."),
    }; 
 
    let mut all_moves: HashMap<i32,Vec<i32>> = HashMap::new();
    while pawns != 0{ 
        let sq = pawns.trailing_zeros() as i32; //gives LSB
        //subbing one makes lsb and all prev bits flip,
        //so setting the board to be the & of that removes lsb
        pawns &= pawns - 1; 
        all_moves.insert(sq,pawn_possible_moves(board, sq));
    } 
    while knights != 0{ 
        let sq = knights.trailing_zeros() as i32; //gives LSB
        knights &= knights - 1;
        all_moves.insert(sq,knight_possible_moves(board, sq));
    } 
    while bishops != 0{ 
        let sq = bishops.trailing_zeros() as i32; //gives LSB
        bishops &= bishops - 1;
        all_moves.insert(sq,bishop_possible_moves(board, sq));
    } 
    while rooks != 0{ 
        let sq = rooks.trailing_zeros() as i32; //gives LSB
        rooks &= rooks - 1;  
        all_moves.insert(sq,rook_possible_moves(board, sq));
    } 
    while queens != 0{
        let sq = queens.trailing_zeros() as i32; //gives LSB
        queens &= queens - 1;
        all_moves.insert(sq,queen_possible_moves(board, sq));
    }
    //i iterate over 1 king in case i want to add weird game modes later on
    if !include_king{return all_moves;}
    while king != 0{
        let sq = king.trailing_zeros() as i32; //gives LSB
        king &= king - 1;
        all_moves.insert(sq,king_possible_moves(board, sq));
    }
    all_moves
}
pub fn is_legal(board: &Bitboard, from: i32, to: i32) -> bool {
    let mut n_board = board.clone();
    n_board.move_piece(from, to);
    n_board.to_move *= -1; // switch to opponent's perspective

    let king_pos = match board.to_move {
        1  => n_board.wk.trailing_zeros() as i32,
        -1 => n_board.bk.trailing_zeros() as i32,
        _ => panic!("Invalid color"),
    };

    // if king is off board something went very wrong
    if king_pos > 63 { return false; }

    !player_possible_moves(&n_board, false)
        .into_iter()
        .any(|(_, v)| v.contains(&king_pos))
}
pub fn player_legal_moves(board: &Bitboard) -> HashMap<i32,Vec<i32>>{
    
    //get all possible moves
    let include_king = true;
    let moves = player_possible_moves(board, include_king);
    
    let mut legals: HashMap<i32,Vec<i32>> = HashMap::new();
    
    //for each piece and its moves:
    for (from,tos) in moves.into_iter(){
        
        //filter the moves to only legal ones
        let piece_legals: Vec<i32> = tos
            .into_iter()
            .filter(|to| is_legal(board, from, *to))
            .collect();
        
        //insert into new map the legal ones
        legals.insert(from,piece_legals);
    }
    legals

}

//can check for each team by changing the turn for lookup then swapping it back to original state
pub fn has_moves(board: &mut Bitboard,c: i32) -> bool{
    if c.abs() != 1{panic!("Invalid color");}
    let prev_c = board.to_move;
    board.to_move = c;
    let moves = player_legal_moves(board);
    board.to_move = prev_c;
    !moves.is_empty()
}

pub fn is_promotion(board: &Bitboard, fromto: (i32,i32)) -> bool{
    if board.piece_at(fromto.0).abs() != 1{ return false; } 
    let (last_rank,second_last_rank) = match board.to_move{
        1 => (7,6),
        -1 => (0,1),
        _ => panic!("invalid color"),
    };
   fromto.0 / 8 == second_last_rank && fromto.1 == last_rank 
 }

pub fn is_checked(_board: &Bitboard, c: i32) -> bool{
    let mut board = _board.clone();
    board.to_move = -c;
    //will grab first king it sees from corresponding array
    let king = match c{
        1 => board.wk,
        -1 => board.bk,
        _ => panic!("Invalid color"),
    };
    let king_pos = king.trailing_zeros() as i32;
    player_legal_moves(&board)
        .into_iter()
        .any(|(_k,v)| {v.contains(&king_pos)})
}

pub fn is_capture(board: &Bitboard,fromto: (i32,i32)) -> bool{
        
    //0 ally, 1 emtpy, 2 enemy
    
    let team = (board.color_of(fromto.1) - board.to_move).abs();
    match team{
        0 => return true,
        1 => return false,
        2 => return true,
        _ => panic!("Team identification via color of subtract is wrong."),
    }

}

fn check_insufficiente_material(board: &Bitboard) -> bool {
    
    if board.wp > 0 {return false;}
    if board.bp > 0 {return false;}
    if board.wr > 0 {return false;}
    if board.br > 0 {return false;}
    if board.wq > 0 {return false;}
    if board.bq > 0 {return false;}

    if board.wn > 0 && board.wb > 0 {return false;}
    if board.bn > 0 && board.bb > 0 {return false;}
    
    let max_white = std::cmp::max(board.wn.count_ones(),board.wb.count_ones());
    let max_black = std::cmp::max(board.bn.count_ones(),board.bb.count_ones());

    if max_white > 0 && max_black > 0 {return false;}
    if max_white > 1 || max_black > 1 {return false;}

    true
}


pub fn board_state(board: &Bitboard,states: &HashMap<[u64;12],i32>) -> i32{
    
    let mut mate = false;
    let can_move = player_legal_moves(board).into_iter().any(|(_,v)| {!v.is_empty()});
    let is_checked = is_checked(board, board.to_move);
    // print!("is checked: {is_checked}");
    // print!("can move: {can_move}");
    if is_checked && !can_move { mate = true; }

    if mate{
        match board.to_move{
            1 => return -1,
            -1 => return 1,
            _ => panic!("Invalid color"),
            
        }
    }

    //for stalemate:
    //no legal moves
    println!("Is checked: {is_checked} | can move {can_move}");
    if !is_checked && !can_move{ return 2; }
    
    //50 moves (50 white 50 black) without captures
    if board.counter >= 100 { return 2; }
    
    //insufficient material:
    // King v King
    // King & Bishop v King
    // King & Knight v King
    let insufficient_material = check_insufficiente_material(board);
    if insufficient_material { return 2; } 

    //3 board repetitions
    let repeated = states.into_iter().any(|(_s,v)| { *v > 2});
    if repeated {return 2;}

    0
}

//given a piece and its new location, trusting its right by other code checking for it
pub fn move_to_notation(board: &mut Bitboard,piece: i32,new_pos: i32,promoted_to: i32)-> String{
    

    let mut notation = String::new();

    let initial: char = match board.piece_at(piece).abs() {
        1 => 'p', //unused
        2 => 'N',
        3 => 'B',
        4 => 'R',
        5 => 'Q',
        6 => 'K',
        _ => panic!("Invalid piece at: {piece}"),
    };

    //pawn 
    if initial == 'p'{

        let capture = is_capture(board, (piece,new_pos));

        let en_pas = (piece % 8 != new_pos % 8) && board.piece_at(new_pos) == 0;
        
        let from_char = board.pos_to_letter(piece);
        let to_num = new_pos / 8; //dont ask why im doing this here instead of bitboard, i dont know.


        if capture || en_pas{

            let to_char = board.pos_to_letter(new_pos);
            
            //exd7
            notation.push_str( &format!("{}x{}{}",from_char,to_char,to_num + 1));  
        }   

        else{
            notation.push_str( &format!("{}{}",from_char,to_num + 1));  
        }
        
        //queen,rook,knight,bishop in ascending order from 1
        match promoted_to{
            0 => {},
            1 => notation.push_str("=Q"),   
            2 => notation.push_str("=R"),
            3 => notation.push_str("=N"),
            4 => notation.push_str("=B"),
            _ => panic!("Invalid promotion code"),
        }
    }


    else{

    notation.push(initial);
    let declare_h = horizontal_ambiguity(board, piece, new_pos);
    let declare_v = vertical_ambiguity(board, piece, new_pos);

    if declare_h {notation.push( board.pos_to_letter(piece) );}
    if declare_v {notation.push_str( &format!("{}",(piece / 8)  + 1) );}
    
    notation.push_str( &format!("{}{}", board.pos_to_letter(new_pos), (new_pos / 8) + 1));

    }
    

    //log happens before move, so create a clone of board 
    //to recreate next state to check if its mate or just a check
    let mut board_clone = board.clone();
    board_clone.move_piece(piece, new_pos);
    board_clone.to_move *= -1;
    // hashmap of states does not matter here so just create an empty one
    let foo: HashMap<[u64;12],i32> = HashMap::new(); 
    
    let state = board_state(&board_clone,&foo);

    //if puts in check, add +
    let checked = is_checked(&board_clone, board_clone.to_move);

    println!("STATE: {state} | CHECKED: {checked} | c: {}",board_clone.to_move);
    render(&board_clone);
    if checked && state == 0 {notation.push('+');} 
    //if its checkmate, add #
    else if is_checked(&board_clone,board_clone.to_move) && state.abs() == 1 {notation.push('#')}

    notation

}   


fn vertical_ambiguity(board: &Bitboard, piece: i32,dest: i32) -> bool{
    
    let p_type = board.piece_at(piece);
    player_legal_moves(board)
        .into_iter()
        .any(|(p,moves)| {
            p != piece && board.piece_at(p) == p_type && moves.contains(&dest) && (piece % 8 == p % 8)
        })   

}

fn horizontal_ambiguity(board: &Bitboard, piece: i32,dest: i32) -> bool{

    let p_type = board.piece_at(piece);
    player_legal_moves(board)
        .into_iter()
        .any(|(p,moves)| {
            p != piece && board.piece_at(p) == p_type && moves.contains(&dest) && (piece / 8 == p / 8)
        })   
}
