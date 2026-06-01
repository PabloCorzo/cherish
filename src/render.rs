use crate::bitboard::Bitboard;
        
 fn get_icon(board: &Bitboard, pos: i32) -> char {
    match board.piece_at(pos) {
        -1 => '♟',
        -2 => '♞',
        -3 => '♗',
        -4 => '♖',
        -5 => '♕',
        -6 => '♔',

         1 => '♙',
         2 => '♘',
         3 => '♝',
         4 => '♜',
         5 => '♛',
         6 => '♚',
         0 => ' ',
        _ => panic!("Piece retrieved was invalid for render"),
    }
}

pub fn render(board: &Bitboard) {
    let mut buf = String::new();
    buf.push_str("  +---+---+---+---+---+---+---+---+\n");

    for rank in (0..8).rev() {           // rank 7 → 0  (black side top, white bottom)
        buf.push_str(&format!("{} ", rank + 1));

        for file in 0..8 {              // file 0 → 7  (a → h, left to right)
            let pos = rank * 8 + file;  // pos 0 = a1 (white's left rook) when rank=0,file=0
            buf.push_str(&format!("| {} ", get_icon(board, pos)));
        }

        buf.push_str("|\n");
        buf.push_str("  +---+---+---+---+---+---+---+---+\n");
    }

    buf.push_str("    a   b   c   d   e   f   g   h\n");
    print!("{}", buf);
}
