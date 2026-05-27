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

pub fn render_aim_train_cli(reverse: bool) -> (i32,i32){

    let mut rng = rand::rng();
    let target_row = rng.random_range(0..8i32);
    let target_col = rng.random_range(0..8i32);
    let mut buffer = String::new();

    buffer.push_str("-----------------------------------------\n");
    for i in 0..8i32{
        buffer.push_str("|");
        for j in 0..8i32{
            // Map render (i,j) to boardstate (row,col) based on perspective:
            // Normal:  rank8 at top, a-file at left  → row = 7-i, col = j
            // Reverse: rank1 at top, h-file at left  → row = i,   col = 7-j
            let (bs_row, bs_col) = if reverse { (i, 7 - j) } else { (7 - i, j) };
            let square = match bs_row == target_row && bs_col == target_col{
              true => "\x1b[42m    \x1b[0m", //Green
              false =>  "    ",
            };
            buffer.push_str(&format!("{}|",square));
        }
    buffer.push_str("\n-----------------------------------------\n");
    }
    println!("{}",buffer);
    (target_row, target_col)
}
