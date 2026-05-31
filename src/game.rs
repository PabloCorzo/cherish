use crate::piece_moves::{board_state, has_moves, is_capture, is_legal, is_promotion};
use crate::bitboard::{Bitboard,letter_to_x};
use crate::render::render; 
use std::io;
use std::io::Write;

#[derive(PartialEq)]
enum GameMode{
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
    state: i32,
}
impl Game{

    pub fn new() -> Self{
        Game{ board: Bitboard::new(), mode: GameMode::Std, state: 0 }
    }
    
    pub fn new_alt(gamemode: GameMode) -> Self{
        Game { board: Bitboard::new(), mode: gamemode, state: 0 }
    }

   pub fn validate_input(&self,board: &Bitboard, input: &str) -> (i32,i32,i32){
       //I.E. e2e4q, where q is piece to promote to.
       //if not promoting and added it will be invalid
       //piece must be: q,b,n,r. any other will not be valid
       //if promoting and not added queen is used as default

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
        let from = 8 * (from_col) + from_row;
        let to = 8 * (to_col) + to_row;


        //accepting awards for ugliest code, please pay accordingly.
        let promotion = is_promotion(board, (from,to));
        if promotion && input.len() == 4 {let input: &str = &format!("{}q",input).to_string();}
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

        (from,to,p)
}


pub fn play_game(&mut self){

    let mut input = "Hello";
    let (mut from, mut to, mut piece) = self.validate_input(&self.board,input);
    let mut valid = from >= 0;
    let mut gamestate = board_state(&self.board);

    while gamestate == 0 {

        //take input, act, change state,loop
        while !valid{
            render(&self.board);
            let input = &get_input();
            let (mut from, mut to, mut piece) = self.validate_input(&self.board,input);

            valid = from >= 0;  
        }

        //has to promote? is there a capture for speed chess?
        let promotion = is_promotion(&self.board, (from,to));
        let capture = is_capture(&self.board, (from,to)); 
 
        //act
        self.board.move_piece(from,to);        
        if self.mode != GameMode::Speed || (self.mode == GameMode::Speed && !capture) {self.board.to_move *= -1;} 

        input = "Hello";
        let mut valid = self.validate_input(&self.board,input).0 >= 0;
    }
}

}
