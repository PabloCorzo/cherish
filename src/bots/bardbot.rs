use crate::bitboard::{Bitboard,letter_to_x};
use crate::bots::opening_book::OpeningBook;
use crate::piece_moves::player_legal_moves;
use crate::bots::randombot::{RandomBot};
use crate::game::GetMove;

use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use serde_json;
use reqwest;
use rand::{SeedableRng};

pub struct BardBot{
    opening_depth: i32,
    pub opening_book: OpeningBook,
    opening_flag: bool,
    chooser:StdRng,
    memorize: bool,
    search_depth: i32,
    from_memory: bool,
}

impl BardBot{

    pub fn new(memorize:bool,from_memory: bool) -> Self{

        BardBot {  
            opening_depth: 10,
            search_depth: 5,
            opening_flag: true,
            memorize,
            chooser: StdRng::from_rng(&mut rand::rng()),
            opening_book: OpeningBook::new(),
            from_memory,
        }
    }


    pub fn _new_alt_depth(opening_depth: i32) -> Self{
        BardBot { 
            opening_depth,
            opening_book: OpeningBook::new(),
            opening_flag: true,
            search_depth: 5,
            memorize: false,
            from_memory: false,
            chooser: StdRng::from_rng(&mut rand::rng()),
        }   
    }


    fn remember_opening(&mut self, board: &Bitboard) -> Option<String> {
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
        panic!("Failed to parse JSON: {}\nBody was: {}\n\nfen was: {}", e, text, fen);
    });
    println!("Called API");

    let moves = json["moves"].as_array().unwrap();
    if moves.is_empty() {
        return None;
    }

    // Score each move by (win rate for the side to move) * (normalized average elo)
    let mut scored: Vec<(usize, f64)> = moves
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let white = m["white"].as_u64().unwrap_or(0) as f64;
            let draws = m["draws"].as_u64().unwrap_or(0) as f64;
            let black = m["black"].as_u64().unwrap_or(0) as f64;
            let total = white + draws + black;

            if total == 0.0 {
                (i, 0.0)
            } else {
                let side_score = if board.to_move == 1 {
                    white + draws * 0.5
                } else {
                    black + draws * 0.5
                };
                let win_rate = side_score / total;
                let avg_elo = m["averageRating"].as_f64().unwrap_or(0.0);
                let elo_norm = (avg_elo / 3000.0).min(1.0);
                (i, (win_rate * 0.7) + (elo_norm * 0.3))
            }
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top: Vec<(usize, f64)> = scored.into_iter().take(5).collect();
    if top.is_empty() || top[0].1 <= 0.0 {
        return None; // nothing usable, engine takes over
    }

    let ratios = [30.0, 25.0, 20.0, 15.0, 10.0];
    let weights: Vec<f64> = (0..top.len()).map(|rank| ratios[rank]).collect();
    let total_weight: f64 = weights.iter().sum();

    let mut pick = rand::random::<f64>() * total_weight;

    for ((idx, _score), &w) in top.iter().zip(weights.iter()) {
        if pick < w {
            let m = &moves[*idx];

            if self.memorize {
                let arr_key: [u64; 12] = [
                    board.wp, board.wr, board.wn, board.wb, board.wq, board.wk,
                    board.bp, board.br, board.bn, board.bb, board.bq, board.bk,
                ];
                let (from, to, promo) = self.parse_move(m["uci"].as_str().unwrap().to_string());

                let entry = self.opening_book.book.entry(arr_key).or_insert_with(Vec::new);
                match entry.iter_mut().find(|v| v.0 == from && v.1 == to && v.2 == promo) {
                    Some(v) => v.3 += 1,
                    None => entry.push((from, to, promo, 0)),
                }
            }

            return Some(m["uci"].as_str().unwrap().to_string());
        }
        pick -= w;
    }

    None
}



    fn parse_move(&self,m: String) -> (i32,i32,i32) {
        

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
    


    fn move_lookahead(&self, board: &Bitboard, n: i32) -> (i32,i32,i32){
        

        let _legals = player_legal_moves(board);
        //minmax search recursively until n is 0
        println!("Choosing at random");
        RandomBot::new().get_move(board)
    }
} 
  
impl GetMove for BardBot{
  
   fn get_move(&mut self,board: &Bitboard) -> (i32,i32,i32){


        if !self.memorize && self.from_memory{
            
            let arr_key:[u64;12] = [board.wp,board.wr,board.wn,board.wb,board.wq,board.wk,board.bp,board.br,board.bn,board.bb,board.bq,board.bk];   
            let book_lookup = self.opening_book.book.get_mut(&arr_key);
            
            match book_lookup{
                Some(move_list) => {
                    println!("{:?}",move_list);
                    let move_tup = *move_list.iter().choose(&mut self.chooser).unwrap();
                    return (move_tup.0,move_tup.1,move_tup.2)
                }, 
                None => return self.move_lookahead(board, self.search_depth),
            }
        }
        

    // SECTION FOR LEARNING OPENINGS THROUGH API CALLS

       if self.opening_flag { 
           let memory = self.remember_opening(board);

            // println!("Memory remembered: {:?}",memory);

           match memory{
                Some(m) => {
                    self.opening_depth -= 1; 
                    if self.opening_depth <= 0 {self.opening_flag = false;}
                    return self.parse_move(m);
                },
                None => {self.opening_flag = false;},
                // None => {self.opening_depth = 0},
           }
       }
  
       //fallback until it actually searches
       RandomBot::new().get_move(board) 
   }

    fn reset(&mut self) {
        self.opening_flag = true;
        self.opening_depth = 10;
    }
} 
  
 
