use crate::bitboard::{Bitboard,letter_to_x};
use crate::bots::opening_book::OpeningBook;
use crate::piece_moves::player_legal_moves;
use crate::bots::randombot::{RandomBot};
use crate::game::GetMove;

use serde_json;
use reqwest;

pub struct BardBot{
    opening_depth: i32,
    opening_book: OpeningBook,
    opening_flag: bool,
    memorize: bool
}

impl BardBot{

    pub fn new(memorize:bool) -> Self{
        BardBot {  
            opening_depth: 10,
            opening_flag: true,
            memorize, 
            opening_book: OpeningBook::new(),
        }
    }


    pub fn _new_alt_depth(opening_depth: i32) -> Self{
        BardBot { 
            opening_depth,
            opening_book: OpeningBook::new(),
            opening_flag: true,
            memorize: false,
        }   
    }


    fn remember_opening(&mut self,board: &Bitboard) -> Option<String> {


        let fen = board.get_fen(); 

        let token = std::env::var("LICHESS_TOKEN").expect("LICHESS_TOKEN not set");

        let resp = reqwest::blocking::Client::new()
            .get("https://explorer.lichess.ovh/masters")
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("fen", fen.clone())])
            .send()
            .unwrap();
        

       let text = resp.text().unwrap();
       let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| {
           panic!("Failed to parse JSON: {}\nBody was: {}\n\nfen was: {}", e, text,fen);
       });
         
        let moves = json["moves"].as_array().unwrap();
        let total_weight: u64 = moves.iter()
        .map(|m| m["white"].as_u64().unwrap() + m["draws"].as_u64().unwrap())
        .sum();


        if total_weight == 0 {
           return None; // empty book, engine takes over
        }
        let mut pick = rand::random::<u64>() % total_weight;

        for m in moves {
            let w = m["white"].as_u64().unwrap() + m["draws"].as_u64().unwrap();
            if pick < w {
                return Some(m["uci"].as_str().unwrap().to_string());
            }
            pick -= w;
        }

        None
    }

    fn parse_move(&mut self,m: String) -> (i32,i32,i32) {
        

        // println!("PARSING: {m}");
        let v: Vec<char> = m.chars().collect();
        let (x1_char,y1_char,x2_char,y2_char,prom_char) = (v[0],v[1],v[2],v[3],v.get(4)); 

        let x1 = letter_to_x(x1_char);
        let x2 = letter_to_x(x2_char);
        let y1 = y1_char as i32 - '1' as i32;
        let y2 = y2_char as i32 - '1' as i32;
        
        let prom = match prom_char{
            Some(c) => {
                match c {
                  'q' => 1,  
                  'r' => 1,  
                  'n' => 1,  
                  'b' => 1,
                  _ => panic!("API returned invalid promotion {c}"),
                }
            }
            None => 0,
        };

        let from = 8 * (y1) + x1;
        let to = 8 * (y2) + x2;

        (from,to,prom)
    }
    

} 
  
impl GetMove for BardBot{
  
   fn get_move(&mut self,board: &Bitboard) -> (i32,i32,i32){
        
       if self.opening_flag { 
           let memory = self.remember_opening(board);

            // println!("Memory remembered: {:?}",memory);

           match memory{
                Some(m) => return self.parse_move(m),
                None => {self.opening_flag = false;},
                // None => {self.opening_depth = 0},
           }
           self.opening_depth =- 1; 
           if self.opening_depth <= 0 {self.opening_flag = false;}
       }
  
       //fallback until it actually searches
       RandomBot::new().get_move(board) 
   }

    fn reset(&mut self) {
        self.opening_flag = true;
        self.opening_depth = 10;
    }
} 
  
 
