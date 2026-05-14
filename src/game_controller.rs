use core::panic;
use std::char;
use std::io::Stdout;

use crate::board::{self, BoardState, Piece, PieceColor, PieceType,get_row_num};
use crate::piece_moves::*;
use crate::render::*;
use crate::input::*;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::terminal;
use ratatui::layout::Rect;
use ratatui::{Frame,Terminal};

//board will have updated both to_move and board. just check if it can move:
//has legal moves
//check playing color or color to play
pub fn is_checkmate(board: &BoardState) -> bool{
    
    //Has moves
    let has_moves = get_player_legal_moves(board, board.to_move).is_empty();

    //Is in check
    let in_check = is_checked(board,board.to_move);

    !has_moves && in_check
}

pub fn is_stalemate(board: &BoardState) -> bool{
    
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
    let checkmate = is_checkmate(board);
    let stalemate = is_stalemate(board);
    match (checkmate,stalemate) {
        (true,true) => panic!("Cant be stalemated and checkmated at once."),
        (true,false) => "checkmate",
        (false,true) => "stalemate",
        (false,false) => "none",
    }
}


pub fn validate_input(board: &BoardState,input: String) -> (bool,[i32;4]){

    let mut arr = [0;4];

    if input.len() != 5 {panic!("move is too short.")}

    let input: String = input.chars().enumerate().map(|(i, c)| {
    if i == 0 && c.is_numeric() {
        char::from_digit(get_row_num(c) as u32, 10).unwrap()
    }else if i == 3 && c.is_numeric(){
        char::from_digit(get_row_num(c) as u32, 10).unwrap()
    }
     else {
        c
    }
}).collect();

    //enforce rule:
    //5 chars that have to be i32 i32 space i32 i32
    for i in input.chars().enumerate(){
        if i.0 == 2 && i.1 != ' '{return (false,arr);}
        else if !i.1.is_numeric(){return (false,arr);}
    }

    //enforce first spot is not empty and second sport does not have piece of same color
    let first_spot = (input.chars().nth(0).unwrap() as i32,input.chars().nth(0).unwrap() as i32);
    let second_spot = (input.chars().nth(1).unwrap() as i32,input.chars().nth(1).unwrap() as i32);

    if board.piece_at(first_spot).t == PieceType::Empty || (board.piece_at(first_spot).c == board.piece_at(second_spot).c){
        return (false,arr)
    }

    arr[0] = first_spot.0;
    arr[1] = first_spot.1;
    arr[2] = second_spot.0;
    arr[3] = second_spot.1;

    (true,arr)

}

pub enum PlayMode{
    Tui,
    Gui,
}
pub struct GameManager{
    board: BoardState,
    config: PlayMode,
    frame: Option<Rect>,  // Frame itself takes a lifetime
}

impl GameManager{     
    pub fn new() -> Self {
        GameManager {
            board: BoardState::new(),
            config: PlayMode::Tui,
            frame: None,   // start with None, assign later
        }
    }

    pub fn new_board(&mut self){
        self.board = BoardState::new();
    }
    pub fn set_config(&mut self,play_mode: &str){

        let mode = match play_mode{
            "tui" => PlayMode::Tui,
            "gui" => PlayMode::Gui,
            &_ => panic!("cant set invalid mode")
        };
        self.config = mode;
    }

    pub fn set_rect(&mut self,rect : Rect){
        self.frame = Some(rect);
    }

    pub fn play_game(&mut self, mut terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>) -> Result<(),String> {
    if is_checkmate(&self.board) || is_stalemate(&self.board) {
        panic!("Game is over before starting");
    }

    let mut state = "none";

    while state == "none" {

        // let valid_input = validate_input(&self.board,"foo".into());
        let s = String::from("foo");
        let mut input = validate_input(&self.board, s);
        while !input.0{
            

            // draw board every input until valid input
            if let Some(ref mut term) = terminal {
            let _ = term.draw(|f| {
                let rect = f.area();
                render_board_tui(f, &self.board, rect);
            });
            }else{
            render_board_gui(&self.board);
            }


            let s = match self.config{
                PlayMode::Gui => input_gui(&self.board),
                PlayMode::Tui => input_tui(),
            };
            input = validate_input(&self.board, s);
        }
        let mut piece = self.board.piece_at((input.1[0],input.1[1],));
        let pos = (input.1[2],input.1[3]);

        state = player_move(&mut self.board, &mut piece, pos);
    }

    Ok(())
}
}
