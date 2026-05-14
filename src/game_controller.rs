use std::io::Stdout;

use crate::board::{self, BoardState, Piece, PieceColor, PieceType,get_row_num};
use crate::piece_moves::*;
use crate::render::*;
use crate::input::*;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::crossterm::terminal;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame,Terminal};

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


pub fn validate_input(board: &BoardState, input: String) -> (bool, [i32; 4]) {
    let mut arr = [0i32; 4];
    let chars: Vec<char> = input.chars().collect();

    // expect "e2 e4" format: file rank space file rank
    if chars.len() != 5 || chars[2] != ' ' { return (false, arr); }

    let (f1, r1, f2, r2) = (chars[0], chars[1], chars[3], chars[4]);

    let valid_file = |c: char| matches!(c.to_ascii_lowercase(), 'a'..='h');
    if !valid_file(f1) || !valid_file(f2) { return (false, arr); }
    if !r1.is_ascii_digit() || !r2.is_ascii_digit() { return (false, arr); }

    let from_col = get_row_num(f1);
    let from_row = r1 as i32 - '1' as i32;
    let to_col   = get_row_num(f2);
    let to_row   = r2 as i32 - '1' as i32;

    if from_row < 0 || from_row > 7 || to_row < 0 || to_row > 7 { return (false, arr); }

    let from_piece = board.piece_at((from_row, from_col));
    if from_piece.t == PieceType::Empty { return (false, arr); }

    let to_piece = board.piece_at((to_row, to_col));
    if to_piece.c == from_piece.c { return (false, arr); }

    arr[0] = from_row; arr[1] = from_col; arr[2] = to_row; arr[3] = to_col;
    (true, arr)
}

pub enum PlayMode{
    Tui,
    Gui,
}
pub struct GameManager{
    board: BoardState,
    config: PlayMode,
    frame: Option<Rect>,  // Frame itself takes a lifetime
    piece_images: PieceImages,
}

impl GameManager{
    pub fn new() -> Self {
        GameManager {
            board: BoardState::new(),
            config: PlayMode::Tui,
            frame: None,
            piece_images: PieceImages::load(),
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
        let mut current_input = String::new();

        while state == "none" {
            if let Some(ref mut term) = terminal {
                // Scope the immutable borrows before we might mutably borrow self.board below.
                {
                    let board_ref = &self.board;
                    let images_ref = &self.piece_images;
                    let turn = match board_ref.to_move {
                        PieceColor::White => "White",
                        PieceColor::Black => "Black",
                        _ => "?",
                    };
                    let status = format!("{} to move  |  Move: {}▌", turn, current_input);

                    term.draw(|f| {
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Min(0), Constraint::Length(3)])
                            .split(f.area());
                        render_board_tui(f, board_ref, chunks[0], images_ref);
                        f.render_widget(
                            Paragraph::new(status.as_str())
                                .block(Block::default().borders(Borders::ALL).title("Input")),
                            chunks[1],
                        );
                    }).unwrap();
                }

                if let Ok(Event::Key(key)) = event::read() {
                    match key.code {
                        KeyCode::Enter => {
                            let inp = validate_input(&self.board, current_input.clone());
                            if inp.0 {
                                let mut piece = self.board.piece_at((inp.1[0], inp.1[1]));
                                let pos = (inp.1[2], inp.1[3]);
                                state = player_move(&mut self.board, &mut piece, pos);
                            }
                            current_input.clear();
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                        KeyCode::Char(c) => { current_input.push(c); }
                        KeyCode::Backspace => { current_input.pop(); }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            } else {
                // GUI mode: render board then wait for a click-based move
                render_board_gui(&self.board);
                let s = input_gui(&self.board);
                let inp = validate_input(&self.board, s);
                if inp.0 {
                    let mut piece = self.board.piece_at((inp.1[0], inp.1[1]));
                    let pos = (inp.1[2], inp.1[3]);
                    state = player_move(&mut self.board, &mut piece, pos);
                }
            }
        }

        Ok(())
    }
}
