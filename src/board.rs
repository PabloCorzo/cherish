
pub struct BoardState{
    pub board: [[Piece;8];8],
    pub en_passant: Option<(i32,i32)>,
    pub to_move: PieceColor,
    //Redundant information but more convenient to go through
    pub playing_pieces: Vec<Piece>,
}
impl BoardState{
    pub fn new() -> Self{

        let mut board = [[Piece::new(PieceType::Empty,PieceColor::Empty,(0,0));8];8];
        let mut playing_pieces = Vec::new();
        //pawn rows
        for i in 0..8{
            board[1][i] = Piece::new(PieceType::Pawn, PieceColor::White, (1,i as i32));
            board[6][i] = Piece::new(PieceType::Pawn, PieceColor::Black, (6,i as i32));
        }
        //white backrank
        board[0][0] = Piece::new(PieceType::Rook, PieceColor::White, (0,0));  
        board[0][7] = Piece::new(PieceType::Rook, PieceColor::White, (0,7));
        board[0][1] = Piece::new(PieceType::Knight, PieceColor::White, (0,1));
        board[0][6] = Piece::new(PieceType::Knight, PieceColor::White, (0,6));
        board[0][2] = Piece::new(PieceType::Bishop, PieceColor::White, (0,2));
        board[0][5] = Piece::new(PieceType::Bishop, PieceColor::White, (0,5));
        board[0][3] = Piece::new(PieceType::Queen, PieceColor::White, (0,3));
        board[0][4] = Piece::new(PieceType::King, PieceColor::White, (0,4));
        //black backrank
        board[7][0] = Piece::new(PieceType::Rook, PieceColor::Black, (7,0));  
        board[7][7] = Piece::new(PieceType::Rook, PieceColor::Black, (7,7));
        board[7][1] = Piece::new(PieceType::Knight, PieceColor::Black, (7,1));
        board[7][6] = Piece::new(PieceType::Knight, PieceColor::Black, (7,6));
        board[7][2] = Piece::new(PieceType::Bishop, PieceColor::Black, (7,2));
        board[7][5] = Piece::new(PieceType::Bishop, PieceColor::Black, (7,5));
        board[7][3] = Piece::new(PieceType::Queen, PieceColor::Black, (7,3));
        board[7][4] = Piece::new(PieceType::King, PieceColor::Black, (7,4));
        
        for i in 0..8{
            playing_pieces.push(board[0][i]); //white backrank
            playing_pieces.push(board[1][i]); //white pawns
            playing_pieces.push(board[6][i]); //black pawns
            playing_pieces.push(board[7][i]); //black backrank
        }


        let board_state = BoardState {
            board,
            en_passant: None,
            to_move: PieceColor::White,
            playing_pieces
        };
        board_state 
    }
    pub fn piece_at(&self,pos: (i32,i32)) -> Piece {
        //if clause for pawns checking diagonally in the edges of the board
        if pos.0 < 0 || pos.0 > 7 ||pos.1 < 0 || pos.1 > 7 {return Piece { t: PieceType::Empty, c: PieceColor::Empty,pos: (-1,-1), castle_rights: false };}  
        self.board[pos.0 as usize][pos.1 as usize]
    }
    pub fn move_piece(&mut self, piece: &mut Piece, pos: (i32,i32)){
        let old_pos = piece.pos;
        self.board[old_pos.0 as usize][old_pos.1 as usize] = Piece::new(PieceType::Empty, PieceColor::Empty, old_pos);
        piece.pos = pos;
        self.board[pos.0 as usize][pos.1 as usize] = *piece;
        // keep playing_pieces in sync: replace old entry with updated piece
        if let Some(p) = self.playing_pieces.iter_mut().find(|p| p.pos == old_pos) {
            *p = *piece;
        }
    }

}
#[derive(Debug,Copy,Clone)]
pub struct Piece{
    pub t: PieceType,
    pub c: PieceColor,
    //position is redundant since multiple pieces with same info need to be differentiated
    pub pos: (i32,i32),
    pub castle_rights: bool,
}
impl Piece{
    pub fn new(t: PieceType,c: PieceColor, pos: (i32,i32)) -> Self{
        let castle_rights = if t == PieceType::King || t == PieceType::Rook {true} else {false};
        Piece{
            t,
            c,
            pos,
            castle_rights,
        }
    }

    pub fn oppose(&mut self) -> PieceColor{
        if self.c == PieceColor::Empty {panic!("Tried to get opposing color of empty square");}
        let color = if self.c == PieceColor::Black {PieceColor::White} else {PieceColor::Black};
        color
    }
}
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum PieceColor{
    Black,
    White,
    Empty
}
impl PieceColor{
    pub fn oppose(&self) -> Self{
        match self{
            Self::Black => PieceColor::White,
            Self::White => PieceColor::Black,
            Self::Empty => PieceColor::Empty,
        }
    }
}



#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty,
}

pub fn get_icon(piece: &Piece) -> char{
    let icon = match (piece.t,piece.c){
        (PieceType::Pawn,PieceColor::White) => '♙',
        (PieceType::Rook,PieceColor::White) => '♖',
        (PieceType::Knight,PieceColor::White) => '♘',
        (PieceType::Bishop,PieceColor::White) => '♗',
        (PieceType::Queen,PieceColor::White) => '♕',
        (PieceType::King,PieceColor::White) => '♔',
        (PieceType::Pawn,PieceColor::Black) => '♟',
        (PieceType::Rook,PieceColor::Black) => '♜',
        (PieceType::Knight,PieceColor::Black) => '♞',
        (PieceType::Bishop,PieceColor::Black) => '♝',
        (PieceType::Queen,PieceColor::Black) => '♛',
        (PieceType::King,PieceColor::Black) => '♚',
        (PieceType::Empty,_)  => ' ',
        (_,PieceColor::Empty) => panic!("Piece has no color? how do you manage that"),
};
    icon
}

pub fn get_row_num(row: char) -> i32{
    let n: i32 = match row.to_ascii_uppercase() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
         _ => panic!("You cant just make up rows mate"),
    };
    n
}

pub fn get_row_char(row: i32) -> char{
    let c: char = match row{
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("You cant just make up rows mate"),
    };
    c
}

pub fn to_algebraic(pos: (i32,i32)) -> (i32,char){
    (pos.0 + 1,get_row_char(pos.1))
}

