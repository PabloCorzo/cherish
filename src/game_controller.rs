use crate::board::{BoardState, Piece, PieceColor, PieceType,get_row_num,to_algebraic};
use crate::piece_moves::*;
use crate::render::*;
use crate::input::*;

use std::fs::{File,OpenOptions};
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};


//board will have updated both to_move and board. just check if it can move:
//has legal moves
//check playing color or color to play
pub fn is_checkmate(board: &BoardState) -> bool{

    let no_moves = get_player_legal_moves(board, board.to_move).is_empty();
    let in_check = is_checked(board,board.to_move);

    no_moves && in_check
}

pub fn is_stalemate(board: &BoardState) -> bool{

    let no_moves = get_player_legal_moves(board, board.to_move).is_empty();
    let in_check = is_checked(board,board.to_move);

    // println!("legal moves: {:?}",get_player_legal_moves(board, board.to_move));

    no_moves && !in_check

    //======================TODO======================//
    //================================================//
    //================================================//
    //              x moves w no captures             //
    //              board repeats 3 times             //
}



//assumes given move is valid. use wisely or it will break boardstate!
fn player_move(board: &mut BoardState, piece: &mut Piece, pos: (i32,i32)) -> &'static str{
    
    if piece.c == PieceColor::Empty || piece.t == PieceType::Empty{panic!("Tried to move either a colorless piece or empty square")} 

    //en passant in state for move checks next turn
    let allows_en_passant = piece.t == PieceType::Pawn && 
    ((piece.pos.0 - pos.0 == 2) || 
    (piece.pos.0 - pos.0 == -2)); 
    
    let en_passant_dir = match piece.c{
        PieceColor::Black => 1,
        PieceColor::White => -1,
        _ => 0,
    };
    //match allows_en_passant {
    //    true => board.en_passant = Some((pos.0 + en_passant_dir,pos.1)),
    //    false => board.en_passant = None,
    //}

    //is it a castle?
    let is_short_castle = piece.t == PieceType::King && piece.pos.1 - pos.1 == -2;
    let is_long_castle = piece.t == PieceType::King && piece.pos.1 - pos.1 == 2;
    //println!("Short castle:{} | Long castle:{}",is_short_castle,is_long_castle);
    let mut new_rook_x: i32 = 0;
    let mut rook_x: i32 = 0;
    if is_long_castle{
        new_rook_x = 3;
        rook_x = 0;
    }
    else if is_short_castle{
        new_rook_x = 5;
        rook_x = 7;
    }

    if is_long_castle || is_short_castle{
        board.move_piece(&mut board.piece_at((piece.pos.0,rook_x)), (piece.pos.0,new_rook_x));
    } 
        

    //castling rights revoke
    piece.castle_rights = false;

    //is capture? if so, remove piece from vector
    let dest_piece = board.piece_at(pos);
    //println!("EN PASSANT: {:?}| POS: {:?}", board.en_passant.unwrap_or((10,10)),pos);
    if pos == board.en_passant.unwrap_or((10,10)) && piece.t == PieceType::Pawn{
        //is an en passant capture!
        let en_passant_dir = match piece.c{
            PieceColor::Black => 1,
            PieceColor::White => -1,
            _ => panic!("Pawn has no color"),
        };

        board.playing_pieces = board.playing_pieces
            .iter()
            .filter(|p| p.pos != (pos.0 + en_passant_dir,pos.1))
            .cloned()
            .collect();
        //println!("CLEARING {:?}",(pos.0+en_passant_dir,pos.1));
        board.board[(pos.0 + en_passant_dir) as usize][pos.1 as usize] = Piece::new(PieceType::Empty, PieceColor::Empty, (pos.0 + en_passant_dir,pos.1));
    }
    else{
        match dest_piece.c == piece.c{
        true => {panic!("Cannot move to place occupied by same color piece.")},
        false => {
            if dest_piece.c != PieceColor::Empty{
            board.playing_pieces = board.playing_pieces
            .iter()
            .filter(|p| p.pos != pos)
            .cloned()
            .collect();
            }
        },
    }
    }
    // now that piece might have been removed, you should move the piece!
    //this fn handles:
    //leaving original spot empty -> placing piece in destination and updating piece position
    //rest stays here due to separation of concerns, game logic goes here
    board.move_piece(piece, pos);


    //switch turn 
    board.to_move = board.to_move.oppose();

    // determine result
    let checkmate = is_checkmate(board);
    let stalemate = is_stalemate(board);
    match allows_en_passant {
        true => board.en_passant = Some((pos.0 + en_passant_dir,pos.1)),
        false => board.en_passant = None,
    }
    match (checkmate,stalemate) {
        (true,true) => panic!("Cant be stalemated and checkmated at once."),
        (true,false) => "checkmate",
        (false,true) => "stalemate",
        (false,false) => "none",
    }
}

pub fn validate_input(board: &BoardState, input: String) -> (bool, [i32; 5]) {
    let mut arr = [0i32; 5];
    let chars: Vec<char> = input.chars().collect();

    // expect "e2 e4" format: file rank space file rank
    if chars.len() < 5 || chars[2] != ' ' { return (false, arr); }

    //from and to
    let (f1, r1, f2, r2) = (chars[0], chars[1], chars[3], chars[4]);

    //goes letter num letter num
    let valid_file = |c: char| matches!(c.to_ascii_lowercase(), 'a'..='h');
    if !valid_file(f1) || !valid_file(f2) { return (false, arr); }
    if !r1.is_ascii_digit() || !r2.is_ascii_digit() { return (false, arr); }

    //transform to numbers
    let from_col = get_row_num(f1);
    let from_row = r1 as i32 - '1' as i32;
    let to_col   = get_row_num(f2);
    let to_row   = r2 as i32 - '1' as i32;

    //bounds check
    if from_row < 0 || from_row > 7 || to_row < 0 || to_row > 7 { return (false, arr); }

    //theres a piece there of player color
    let from_piece = board.piece_at((from_row, from_col));
    if from_piece.t == PieceType::Empty || from_piece.c != board.to_move{ return (false, arr); }

    //there is no ally piece on dest
    let to_piece = board.piece_at((to_row, to_col));
    if to_piece.c == from_piece.c { 
        //println!("Cant move to square with ally piece");
        return (false, arr);
    }

    arr[0] = from_row; arr[1] = from_col; arr[2] = to_row; arr[3] = to_col;
    
    //if is a promotion extract the piece to be promoted to
    if is_promotion(&board, (arr[0],arr[1]), (arr[2],arr[3])){
        if chars.len() < 6 {return (false,arr);}
        arr[4] = match chars[6]{
            'q' => 1,
            'r' => 2,
            'k' => 3,
            'b' => 4,
            _ => 0,
        };
        if arr[4] == 0 {return (false, arr)}
    }
    
    let valid_moves = get_player_legal_moves(board,board.to_move);
    match valid_moves.get(&(arr[0],arr[1])){
        None => {
            //println!("unwrap was none for {:?}",(arr[0],arr[1]));
            return (false, arr);
        },
        Some(moves) => {
            if moves.iter().any(|pos| pos == &(arr[2],arr[3])){
                    println!("Move is {:?}",arr);
                    return (true, arr);
                }
        }
    }
    //println!("There were no valid moves that matched");
    (false, arr)
}

pub struct GameManager{
    board: BoardState,
    log_file: Option<File>,
    move_count: i32,
}

impl GameManager{
    pub fn new() -> Self {
        GameManager {
            board: BoardState::new(),
            log_file: None,
            move_count: 0,
        }
    }

    // pub fn new_board(&mut self){
    //     self.board = BoardState::new();
    // }

    
    fn get_promoted_piece(&self,moves: String) -> Option<PieceType>{
        let chars: Vec<char> = moves.chars().collect();
        match chars[6]{
            'q' => Some(PieceType::Queen),
            'r' => Some(PieceType::Rook),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            _ => None,
        }
    }

    fn log(&mut self, moves: ((i32,i32),(i32,i32)),promoted_to: Option<PieceType>){
        let file = self.log_file.as_mut().expect("log called but no log file open");
        let piece = &mut self.board.piece_at(moves.0);
        let str = move_to_notation(&mut self.board, piece, moves.1,promoted_to);
        let mut writer = BufWriter::new(file);
        let line = format!("{}:{}", char::from_digit(self.move_count as u32, 10).unwrap(), str);
        writeln!(writer, "{}", line).expect("Could not log move.");
    }

    pub fn play_game(&mut self, log: bool) -> Result<(),String> {

        if log && self.log_file.is_none() {
            self.log_file = Some(
                OpenOptions::new().create(true).write(true).truncate(true)
                    .open("game.log").expect("could not open log file")
            );
        }

        if is_checkmate(&self.board) || is_stalemate(&self.board) {
            panic!("Game is over before starting");
        }

        let mut state = "none";

        let mut moves: [i32;5] = [0;5];
        //loop until game is done
        while state == "none"{
            
            self.show_valid_moves();
            //get input
            let mut input = String::from("__ __");
            let mut valid = validate_input(&self.board, input.clone()).0;
            while !valid{
                //print board
                render_board_cli(&self.board);
                input = input_tui();
                (valid,moves) = validate_input(&self.board, input.clone());
            }
            let turn = ((moves[0],moves[1]),(moves[2],moves[3]));
            let mut piece = self.board.piece_at(turn.0);

            let will_promote = is_promotion(&self.board, turn.0, turn.1);
            let promoted_to: Option<PieceType>;
            if will_promote{
                promoted_to = self.get_promoted_piece(input);
            }else{promoted_to = None}
            if log{self.log(turn,promoted_to);}
            state = player_move(&mut self.board,&mut piece, turn.1);

            //promotion flag has something
            if moves[4] != 0{
                let t = match moves[4]{
                    0 => panic!("mismatch between type integer mapping"),
                    1 => PieceType::Queen,
                    2 => PieceType::Rook,
                    3 => PieceType::Knight,
                    4 => PieceType::Bishop,
                    _ => panic!("mismatch between type integer mapping"),
                };
                self.board.promote_piece((moves[2],moves[3]), t);
            }else {
            }
            
            self.move_count = self.move_count + 1;
        }

        Ok(())      
    
}

fn show_valid_moves(&self){
    let valid_moves = get_player_legal_moves(&self.board, self.board.to_move);

    for (pos, moves) in &valid_moves {
        let piece = self.board.piece_at(*pos);
        let (rank, file) = to_algebraic(*pos);
        let move_strs: Vec<String> = moves.iter().map(|m| {
            let (mr, mf) = to_algebraic(*m);
            format!("{}{}", mf, mr)
        }).collect();
        println!("{:?} at {}{} : {}", piece.t, file, rank, move_strs.join(", "));
    }
}

fn validate_aim_input(input: String) -> (bool,(i32,i32)){
    
    let mut valid = false;
    let mut pos = (-1,-1);
    let valid_len = input.len() == 2;
    if !valid_len{ return (valid,pos);}

    let valid_file = |c: char| matches!(c.to_ascii_lowercase(), 'a'..='h');
    let chars: Vec<char> = input.chars().collect();
    let (f,r) = (chars[0],chars[1]);
    let valid_x = valid_file(f);
    if  !r.is_ascii_digit(){return (valid,pos);}
    let row = r as i32 - '1' as i32;
    let file = get_row_num(f);
    let valid_y = row <= 8 && row >= 0;
    if valid_x && valid_y {
        valid = true;
        pos = (row,file);
    }
    (valid,pos)
} 

pub fn play_aim_game(&self) -> Result<(),String>{
    
    loop{
        let (x,y) = render_aim_train_cli();
        loop{
        let mut input = String::from("foo");
        let (mut valid,mut pos) = GameManager::validate_aim_input(input);
        while !valid{
            input = input_tui();
            (valid,pos) = GameManager::validate_aim_input(input);
            }
        //println!("POS:{:?} X,Y:{:?}",pos,(x,y));
        if pos == (7 - y, x){break;}
    }
    }
}

}
