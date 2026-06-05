use rand::seq::SliceRandom;

fn move_bits(bitmap: u64, from: i32, to: i32) -> u64 {
    if to >= from {
        bitmap << (to - from)
    } else {
        bitmap >> (from - to)
    }
}

#[derive(Clone)]
pub struct Bitboard{
    
    // White pieces
    pub wp : u64,    
    pub wr : u64,    
    pub wn : u64,    
    pub wb : u64,    
    pub wq : u64,    
    pub wk : u64,    
    
    // Black pieces
    pub bp : u64,    
    pub br : u64,    
    pub bn : u64,    
    pub bb : u64,    
    pub bq : u64,    
    pub bk : u64,    
    pub to_move: i32,
    pub en_passant: Option<i32>,
    pub castle_rights: Vec<i32>,
    pub counter: i32,
}
impl Bitboard{
    pub fn new() -> Self{
        
        let mut castle_rights: Vec<i32> = Vec::new();
        
        castle_rights.push(0);
        castle_rights.push(7);
        castle_rights.push(56);
        castle_rights.push(63);
        castle_rights.push(4);
        castle_rights.push(60);
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
            castle_rights,
            counter: 0,
        }    
     
    }     
    pub fn new_960() -> Self{

        let mut board = Bitboard::new();

        board.wp = 0xFF << 8;
        board.bp = 0xFF << 48;

        // piece codes: 1=rook 2=knight 3=bishop 4=queen 5=king
        let mut backrank: [i32; 8] = [1, 1, 2, 2, 3, 3, 4, 5];
        backrank.shuffle(&mut rand::rng());

        board.wr = 0; board.wn = 0; board.wb = 0; board.wq = 0; board.wk = 0;
        board.br = 0; board.bn = 0; board.bb = 0; board.bq = 0; board.bk = 0;
        board.castle_rights.clear();

        for (i, &piece) in backrank.iter().enumerate() {
            let wsq = i as u64;
            let bsq = (56 + i) as u64;
            match piece {
                1 => { board.wr |= 1u64 << wsq; board.br |= 1u64 << bsq;
                       board.castle_rights.push(i as i32); board.castle_rights.push((56 + i) as i32); }
                2 => { board.wn |= 1u64 << wsq; board.bn |= 1u64 << bsq; }
                3 => { board.wb |= 1u64 << wsq; board.bb |= 1u64 << bsq; }
                4 => { board.wq |= 1u64 << wsq; board.bq |= 1u64 << bsq; }
                5 => { board.wk |= 1u64 << wsq; board.bk |= 1u64 << bsq;
                       board.castle_rights.push(i as i32); board.castle_rights.push((56 + i) as i32); }
                _ => unreachable!(),
            }
        }

        board
    }      
    pub fn piece_at(&self,square: i32) -> i32{
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
   pub fn move_piece(&mut self, pos: i32, new_pos: i32) {
    let piece = self.piece_at(pos as i32);
    if piece == 0 { panic!("Tried to move none piece at {}", pos); }
    
    let castle = piece.abs() == 6 && self.piece_at(new_pos).abs() == 4;
        
    if piece.abs() == 6 || piece.abs() == 4 {self.castle_rights.retain(|&p| { p != pos} );}


    let team = (self.color_of(new_pos) - self.to_move).abs();
    if team == 2 { self.counter = 0; }
    else { self.counter += 1 }

    if castle {
        let goes_right = new_pos > pos;
        let king_dest = if goes_right { pos + 2 } else { pos - 2 };
        let rook_dest = if goes_right { pos + 1 } else { pos - 1 };

        self.move_piece(new_pos, rook_dest);

        let king_bitmap = match self.to_move {
            1  => &mut self.wk,
            -1 => &mut self.bk,
            _ => panic!("Invalid color"),
        };
        let bit = *king_bitmap & (1u64 << pos as u32);
        let moved = move_bits(bit, pos, king_dest);
        *king_bitmap = (*king_bitmap & !(1u64 << pos as u32)) | moved;
        return;
    }

    // clear the captured piece's bit before moving the attacker
    let target = self.piece_at(new_pos);
    if target != 0 {
        let captured = match target {
            1  => &mut self.wp,  -1 => &mut self.bp,
            2  => &mut self.wn,  -2 => &mut self.bn,
            3  => &mut self.wb,  -3 => &mut self.bb,
            4  => &mut self.wr,  -4 => &mut self.br,
            5  => &mut self.wq,  -5 => &mut self.bq,
            6  => &mut self.wk,  -6 => &mut self.bk,
            _ => panic!("Invalid piece at capture target"),
        };
        *captured &= !(1u64 << new_pos as u32);
    }

    let piece_bitmap = match piece {
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
        _ => panic!("When moving piece received an invalid case"),
    };

    let piece_bit = *piece_bitmap & (1u64 << pos as u32);
    let moved = move_bits(piece_bit, pos, new_pos);
    *piece_bitmap = (*piece_bitmap & !(1u64 << pos as u32)) | moved;
   }
    
    // Do abs of color_of - c, result will be:
    //     0 for ally
    //     1 for empty
    //     2 for enemy
    pub fn color_of(&self,pos: i32) -> i32{
        let piece = self.piece_at(pos);
        if piece == 0 {return 0;}
        match piece > 0{
            true => return 1,
            false => return -1,
        }
    }

    pub fn promote_to(&mut self, pawn: i32, piece: i32) {
        match (self.to_move, piece) {
            (1, 1)  => { self.wp &= !(1u64 << pawn as u32); self.wq |= 1u64 << pawn as u32; }
            (1, 2)  => { self.wp &= !(1u64 << pawn as u32); self.wr |= 1u64 << pawn as u32; }
            (1, 3)  => { self.wp &= !(1u64 << pawn as u32); self.wn |= 1u64 << pawn as u32; }
            (1, 4)  => { self.wp &= !(1u64 << pawn as u32); self.wb |= 1u64 << pawn as u32; }
            (-1, 1) => { self.bp &= !(1u64 << pawn as u32); self.bq |= 1u64 << pawn as u32; }
            (-1, 2) => { self.bp &= !(1u64 << pawn as u32); self.br |= 1u64 << pawn as u32; }
            (-1, 3) => { self.bp &= !(1u64 << pawn as u32); self.bn |= 1u64 << pawn as u32; }
            (-1, 4) => { self.bp &= !(1u64 << pawn as u32); self.bb |= 1u64 << pawn as u32; }
            _ => panic!("Invalid promotion"),
        }
    }

    pub fn pos_to_letter(&self,pos: i32) -> char{

        match pos % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("mod 8 broke"),
        }
    }
}
pub fn letter_to_x(letter: char) -> i32{
    match letter{
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid file"),
    }
}
