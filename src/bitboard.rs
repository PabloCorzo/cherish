fn move_bits(bitmap: u64, from: i32, to: i32) -> u64 {
    if to >= from {
        bitmap << (to - from)
    } else {
        bitmap >> (from - to)
    }
}

struct Bitboard{
    
    // White pieces
    wp : u64,    
    wr : u64,    
    wn : u64,    
    wb : u64,    
    wq : u64,    
    wk : u64,    
    
    // Black pieces
    bp : u64,    
    br : u64,    
    bn : u64,    
    bb : u64,    
    bq : u64,    
    bk : u64,    
    to_move: i32,
    en_passant: Option<u32>,
    
}
impl Bitboard{
    fn new() -> Self{

        Bitboard{
            
            //Think of sets of 4b. 
            // '<< 8*n' moves the value n rows up.

            //WHITE
            wp: 0xFF << 8,
            wr: 0x81,
            wn: 0x42,
            wb: 0x24,
            wq: 0x08,
            wk: 0x10,

            //BLACK
            bp: 0xFF << 48,
            br: 0x81 << 56,
            bn: 0x42 << 56,
            bb: 0x24 << 56,
            bq: 0x08 << 56,
            bk: 0x10 << 56,
           
            en_passant: None,
            to_move:1,
        }    
     
    }     
      
    fn piece_at(&self,square: i32) -> i32{
        if square < 0 || square > 63 {return -1;}
        let square: u32 = square as u32;
        // will return a signed int to map to piece
        // 0 -> empty 1 -> pawn 2 -> knight 3 -> bishop 4 -> rook  5 -> queen 6 -> king
        // + for white, - for black
        if square >= 64 {panic!("Tried to get piece at index {}",square);}
        let mask = 1u64 << square;
        if self.wp & mask != 0 {return 1;}
        if self.bp & mask != 0 {return -1;}
        if self.wn & mask != 0 {return 2;}
        if self.bn & mask != 0 {return -2;}
        if self.wb & mask != 0 {return 3;}
        if self.bb & mask != 0 {return -3;}
        if self.wr & mask != 0 {return 4;}
        if self.br & mask != 0 {return -4;}
        if self.wq & mask != 0 {return 5;}
        if self.bq & mask != 0 {return -5;}
        if self.wk & mask != 0 {return 6;}
        if self.bk & mask != 0 {return -6;}
        0
    }
    

    //Assumes move is valid
    //will panic if pos is empty
    //will replace whatever is on the other end, does not matter if its ally
    fn move_piece(&mut self,pos: i32, new_pos: i32){

        let piece = self.piece_at(pos as i32);
        if piece == 0 {panic!("Tried to move none piece at {}",pos);}
        
        let piece_bitmap = match piece{
            1  => &mut self.wp,
            -1 => &mut self.bp,
            2  => &mut self.wn,
            -2 => &mut self.bn,
            3  => &mut self.wb,
            -3 => &mut self.bb,
            4  => &mut self.wr,
            -4 => &mut self.br,
            5  => &mut self.wq,
            -5 => &mut self.bq,
            6  => &mut self.wk,
            -6 => &mut self.bk,
            _ => panic!("When moving piece recieved and invalid case"),
        };

        // UNLESS SPECIFIED, NUMBERS WILL BE U32, SO DECLARE AS U64 IF ITS BITMAP
        let piece_bit = *piece_bitmap & (1u64 << pos);
        let moved = move_bits(piece_bit,pos,new_pos);

        *piece_bitmap = (*piece_bitmap & !(1u64 << pos)) | moved;
    }
    
    fn color_of(&self,pos: i32) -> i32{
        let piece = self.piece_at(pos);
        if piece == 0 {return 0;}
        match piece > 0{
            true => return 1,
            false => return -1,
        }
    }
}

fn pawn_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();

    let (one_step,two_step): (i32,i32) = match c{
        1  => (piece + 8,piece + 16),
        -1 => (piece - 8,piece - 16),
        _ => panic!("Pawn has invalid color"),
    };
    let (dl,dr): (i32,i32) = match c {
        1  => (piece + 7,piece + 9),
        -1 => (piece - 7,piece - 9),
        _ => panic!("Pawn has invalid color"),
    };

    // Do abs of color_of - c, result will be:
    //     0 for ally
    //     1 for empty
    //     2 for enemy
    //TODO: EN PASSANT CHECK
    let can_jump: bool = match c{
        1  => piece >= 8 && piece <= 15,
        -1 => piece >= 48 && piece <= 55,
        _ => panic!("Invalid pawn color"),
    };
    let team = (board.color_of(one_step) - c).abs();
    match team{
        0 => {},
        1 => moves.push(one_step),
        2 => {},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let team = (board.color_of(two_step) - c).abs();
    match team{
        0 => {},
        1 => if can_jump && !moves.is_empty(){moves.push(two_step);},
        2 => {},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let file = piece % 8;
    let team = (board.color_of(dl) - c).abs();
    match team{
        0 => {},
        1 => {},
        2 => if file > 0 {moves.push(dl)},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    let team = (board.color_of(dr) - c).abs();
    match team{
        0 => {},
        1 => {},
        2 => if file < 7{moves.push(dr)},
        _ => panic!("Team identification via color of subtract is wrong."),
    }

    moves
}


// Knight move offsets, N = piece location (e.g. e4):
//
//   a    b    c    d    e    f    g    h
//   .    .    .   +15   .   +17   .    .    <- rank 6
//   .    .   +6    .    .    .   +10   .    <- rank 5
//   .    .    .    .    N    .    .    .    <- rank 4
//   .    .   -10   .    .    .   -6    .    <- rank 3
//   .    .    .   -17   .   -15   .    .    <- rank 2

fn knight_possible_moves(board: &Bitboard,piece: i32,c: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();
        
    let jumps: [i32;8] = [piece + 6,piece + 15,piece + 10,piece + 17,piece - 10, piece - 17, piece - 6,piece - 15];

    moves
}


fn rook_possible_moves(board: &Bitboard, piece: i32,c: i32) -> Vec<i32>{
    let mut moves: Vec<i32> = Vec::new();
    
    fn move_until_blocked(dir: (i32,i32)) -> Vec<i32>{
        let mut squares: Vec<i32> = Vec::new();
        
        // let mut pos = piece;
        // while pos <= 63 && pos >= 0{
        //
        // }

        squares
    }

    moves
}
