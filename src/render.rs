use crate::board::{BoardState, get_icon};
use rand::{RngExt};

pub fn render_board_cli(board: &BoardState){

    //41 long
    println!("-----------------------------------------");
    for i in 0..8{
        let mut buffer = String::new();
        buffer.push_str("| ");
        for j in 0..8{
            buffer.push_str(&format!("{}  | ", get_icon(&board.piece_at((7 - i,j)))));
        }
        println!("{}",buffer);
        println!("-----------------------------------------");
    }
}

pub fn render_aim_train_cli() -> (i32,i32){

    let mut rng = rand::rng();
    let x = rng.random_range(0..8);
    let y = rng.random_range(0..8);
    let mut buffer = String::new();
    
    buffer.push_str("-----------------------------------------\n");
    for i in 0..8{
        buffer.push_str("|");
        for j in 0..8{
            let square = match i == y && j == x{
              true => "\x1b[42m    \x1b[0m", //Green
              false =>  "    ",
            };
            buffer.push_str(&format!("{}|",square));
        }
    buffer.push_str("\n-----------------------------------------\n");
    }
    println!("{}",buffer);
    (x,y)    
}
