use crate::bitboard::Bitboard;
use std::collections::{HashMap};

fn pawn_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();

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
    let en_passant = match board.en_passant{
        Some(i) => i,
        None => 64,
    };
    
    if en_passant == dr {moves.push(dr);}
    if en_passant == dl {moves.push(dl);}

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

fn knight_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();
        
    let jumps: [i32;8] = [piece + 6,piece - 10,piece + 15,piece - 17,piece + 10, piece - 6, piece + 17,piece - 15];
    let file = piece % 8;
    let left_free = file;
    let right_free = 7 - file;
    let top_free = 7 - (piece / 8);
    let bot_free = piece / 8;

    //2 left, 1 up/down
    if left_free > 1 {
        if top_free > 0{
            match board.color_of(jumps[0]) == c{
            true => moves.push(jumps[0]),
            false => {},
            }
        }
        if bot_free > 0{
            match board.color_of(jumps[1]) == c{
            true => moves.push(jumps[1]),
            false => {},
            }
        }
    }
    //1 left, 2 up/down
    if left_free > 0{
        if top_free > 1{
            match board.color_of(jumps[2]) == c{
            true => moves.push(jumps[2]),
            false => {},
            }
        }
        if bot_free > 1{
            match board.color_of(jumps[3]) == c{
            true => moves.push(jumps[3]),
            false => {},
            }
        }
    }
    //2 right, 1 up/down
    if right_free > 1{
        if top_free > 0{
            match board.color_of(jumps[4]) == c{
            true => moves.push(jumps[4]),
            false => {},
            }
        }
        if bot_free > 0{
            match board.color_of(jumps[5]) == c{
            true => moves.push(jumps[5]),
            false => {},
            }
        }
    }
    //1 right, 2 up/down
    if right_free > 0{
        if top_free > 1{
            match board.color_of(jumps[6]) == c{
            true => moves.push(jumps[6]),
            false => {},
            }
        }
        if bot_free > 1{
            match board.color_of(jumps[7]) == c{
            true => moves.push(jumps[7]),
            false => {},
            }
        }
    }
    moves
}


fn rook_possible_moves(board: &Bitboard, piece: i32,c: i32) -> Vec<i32>{
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

fn bishop_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
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

fn queen_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
    let rm = rook_possible_moves(board, piece, c);
    let bm = bishop_possible_moves(board, piece, c);

    [rm,bm].into_iter().flatten().collect()
}

fn king_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
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
            0 => break,
            1 => moves.push(square),
            2 => {
                moves.push(square);
                break;
            },
            _ => panic!("Team identification via color of subtract is wrong."),
        }

    }
    moves
}
fn player_possible_moves(board: &Bitboard,c: i32,include_king: bool) -> HashMap<i32,Vec<i32>>{
    
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
        all_moves.insert(sq,pawn_possible_moves(board, sq, c));
    }
    while knights != 0{
        let sq = knights.trailing_zeros() as i32; //gives LSB
        knights &= knights - 1;
        all_moves.insert(sq,knight_possible_moves(board, sq, c));
    }
    while bishops != 0{
        let sq = bishops.trailing_zeros() as i32; //gives LSB
        bishops &= bishops - 1;
        all_moves.insert(sq,bishop_possible_moves(board, sq, c));
    }
    while rooks != 0{
        let sq = rooks.trailing_zeros() as i32; //gives LSB
        rooks &= rooks - 1;
        all_moves.insert(sq,rook_possible_moves(board, sq, c));
    }
    while queens != 0{
        let sq = queens.trailing_zeros() as i32; //gives LSB
        queens &= queens - 1;
        all_moves.insert(sq,queen_possible_moves(board, sq, c));
    }
    //i iterate over 1 king in case i want to add weird game modes later on
    if !include_king{return all_moves;}
    while king != 0{
        let sq = king.trailing_zeros() as i32; //gives LSB
        king &= king - 1;
        all_moves.insert(sq,king_possible_moves(board, sq, c));
    }
    all_moves
}
