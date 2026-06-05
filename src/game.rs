use crate::piece_moves::{board_state, is_capture, is_promotion, player_legal_moves,move_to_notation};
use crate::bitboard::{Bitboard,letter_to_x};
use crate::render::render; 
use std::io;
use std::io::Write;
use std::collections::HashMap;

pub fn print_legal_moves(board: &Bitboard) {
    let moves = player_legal_moves(board);
    let mut entries: Vec<(i32, Vec<i32>)> = moves.into_iter().collect();
    entries.sort_by_key(|(sq, _)| *sq);

    println!("Legal moves:");
    for (from, tos) in entries {
        if tos.is_empty() { continue; }
        let from_sq = sq_to_name(from);
        let to_sqs: Vec<String> = tos.iter().map(|&t| sq_to_name(t)).collect();
        println!("  {} -> {}", from_sq, to_sqs.join(", "));
    }
}

fn sq_to_name(sq: i32) -> String {
    let file = (b'a' + (sq % 8) as u8) as char;
    let rank = (sq / 8) + 1;
    format!("{}{}", file, rank)
}
#[derive(PartialEq)]
pub enum GameMode{
    Std,
    N60,
    Speed,
}


pub fn get_input() -> String{                                                                                                                                                                            
    print!("Enter your move: ");                                                                                                                                                                     
    io::stdout().flush().unwrap();                                                                                                                                                                   
                                                                                                                                                                                                     
    let mut input = String::new();                                                                                                                                                                   
    io::stdin().read_line(&mut input).unwrap();                                                                                                                                                      
    let play = input.trim();

    play.into()                                                                                                                                                                                      
                                                                                                                                                                                                       
  }            


pub struct Game{
    board: Bitboard,
    mode: GameMode,
    states: HashMap<[u64;12],i32>,
    minlog: bool,
    record: String,
    counter: u32,
}
impl Game{

    pub fn new(minlog: bool) -> Self{
        Game{ board: Bitboard::new(), mode: GameMode::Std, states: HashMap::new(),minlog,record: String::new(),counter: 0}
    }
   

    pub fn new_alt(gamemode: GameMode,minlog: bool) -> Self{
        
        let board = match gamemode{
            GameMode::N60 => Bitboard::new_960(),
            _ => Bitboard::new(),
        };
        Game { board, mode: gamemode, states: HashMap::new(),minlog,record: String::new(),counter: 0}
    }
    
    pub fn store_position(&mut self){
        
        let arr = [self.board.wp,self.board.wr,self.board.wn,self.board.wb,self.board.wq,self.board.wk,
        self.board.bp,self.board.br,self.board.bn,self.board.bb,self.board.bq,self.board.bk];
        

        *self.states.entry(arr).or_insert(0) += 1;
    }
    
   pub fn validate_input(&self,board: &Bitboard, mut input: String) -> (i32,i32,i32){
       //I.E. e2e4q, where q is piece to promote to.
       //if not promoting and added it will be invalid
       //piece must be: q,b,n,r. any other will not be valid
       //if promoting and not added queen is used as default
       if input.len() < 4 { return (-1, -1, -1); }
       let chars: Vec<char> = input.chars().collect();
       let valid_file = |c: char| matches!(c.to_ascii_lowercase(), 'a'..='h');
       if !chars[1].is_ascii_digit() || !chars[3].is_ascii_digit() {return (-1,-1,-1);}
       if !valid_file(chars[0]) || !valid_file(chars[2]) {return (-1,-1,-1);}


        
        //transform to numbers
        let from_col = chars[0];
        let from_row = chars[1] as i32 - '1' as i32;
        let to_col   = chars[2];
        let to_row   = chars[3] as i32 - '1' as i32;

        //bounds check
        if from_row < 0 || from_row > 7 || to_row < 0 || to_row > 7 { return (-1,-1,-1); }
        let from_col = letter_to_x(from_col);
        let to_col = letter_to_x(to_col);
        let from = 8 * (from_row) + from_col;
        let to = 8 * (to_row) + to_col;
        
        // println!("from col {from_col} | from row {from_row}");
        // println!("to col {to_col} | to row {to_row}");

        //accepting awards for ugliest code, please pay accordingly.
        let promotion = is_promotion(board, (from,to));
        if promotion && input.len() == 4 {input = format!("{}q",input).to_string();}
        if !promotion && input.len() != 4 {return (-1,-1,-1);}
        if promotion && input.len() != 5 {return (-1,-1,-1);}
        

        let mut p = 0;
        if promotion{
            match chars[4]{
                'q' => p = 1,
                'r' => p = 2,
                'n' => p = 3,
                'b' => p = 4,
                _ => return (-1,-1,-1),
            }
        }
        println!("Move is valid: {from},{to},{p}");
        (from,to,p)
}

pub fn play_game(&mut self) {
    let mut from;
    let mut to;
    let mut piece;
    let mut gamestate;
    loop {
        
        gamestate = board_state(&self.board,&self.states);
        // println!("Gamestate is: {gamestate}");
        if gamestate != 0 { break; }

        print_legal_moves(&self.board);
        loop {
            render(&self.board);
            let input = get_input();
            (from, to, piece) = self.validate_input(&self.board, input);
            if from >= 0 { break; }
        }

        let promotion = is_promotion(&self.board, (from, to));
        let capture = is_capture(&self.board, (from, to));



        let legal = player_legal_moves(&self.board)
            .into_iter()
            .any(|(k,v)| { k == from && v.contains(&to)});
        if !legal { continue; }

        //log if flagged as logged game
        if self.minlog{ self.log(from,to,piece); }
        self.counter += 1;

        self.board.move_piece(from, to);
        if promotion { self.board.promote_to(to, piece); }
        if self.mode != GameMode::Speed || (self.mode == GameMode::Speed && !capture) {
            self.board.to_move *= -1;
        }
        self.store_position();
    }
    render(&self.board);    
    match gamestate{
        1 => println!("White wins."),
        -1 => println!("Black wins."),
        2 => println!("Stalemate."),
        _ => panic!("Invalid gamestate."),

    }

    if self.minlog{ println!("{}", self.record);}
}

fn log(&mut self, from:i32, to:i32,promoted_to: i32){
    let notation = move_to_notation(&mut self.board, from, to, promoted_to);
    match self.counter % 2{
        0 => self.record.push_str(&format!("{notation} | ")),
        1 => self.record.push_str(&format!("{notation}\n")),
        _ => panic!("Mod 2 broke.")

    }
}
}
