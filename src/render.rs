use crate::board::{BoardState, get_icon};

pub fn render_board_cli(board: &BoardState){

    println!("-----------------------------------------");
    for i in 0..8{
        //16 long
        // println!("----------------");
        //41
        let mut buffer = String::new();
        buffer.push_str("| ");
        for j in 0..8{
            buffer.push_str(&format!("{}  | ", get_icon(&board.piece_at((7 - i,j)))));
        }
        println!("{}",buffer);
        println!("-----------------------------------------");
    }
}
