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
    pub en_passant: i32,
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
           
            en_passant: -1,
            to_move:1,
            castle_rights,
            counter: 0,
        }    
     
    }    


pub fn new_from_fen(fen: &str) -> Self {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    assert!(parts.len() >= 2, "FEN must have at least piece and side-to-move fields");

    let mut board = Bitboard {
        wp: 0, wr: 0, wn: 0, wb: 0, wq: 0, wk: 0,
        bp: 0, br: 0, bn: 0, bb: 0, bq: 0, bk: 0,
        to_move: 1,
        en_passant: -1,
        castle_rights: Vec::new(),
        counter: 0,
    };

    // ── 1. Piece placement ──────────────────────────────────────────
    // FEN lists ranks 8→1 (top to bottom), files a→h (left to right).
    let mut rank: i32 = 7;
    let mut file: i32 = 0;
    for ch in parts[0].chars() {
        match ch {
            '/' => { rank -= 1; file = 0; }
            '1'..='8' => { file += ch as i32 - '0' as i32; }
            _ => {
                let sq = rank * 8 + file;
                let bit = 1u64 << sq as u32;
                match ch {
                    'P' => board.wp |= bit,
                    'R' => board.wr |= bit,
                    'N' => board.wn |= bit,
                    'B' => board.wb |= bit,
                    'Q' => board.wq |= bit,
                    'K' => board.wk |= bit,
                    'p' => board.bp |= bit,
                    'r' => board.br |= bit,
                    'n' => board.bn |= bit,
                    'b' => board.bb |= bit,
                    'q' => board.bq |= bit,
                    'k' => board.bk |= bit,
                    _ => panic!("Unknown FEN piece char: '{}'", ch),
                }
                file += 1;
            }
        }
    }

    // ── 2. Side to move ─────────────────────────────────────────────
    board.to_move = match parts[1] {
        "w" => 1,
        "b" => -1,
        _ => panic!("Invalid side-to-move in FEN: '{}'", parts[1]),
    };

    // ── 3. Castling rights ──────────────────────────────────────────
    // castle_rights stores the *square* of each king/rook that still has rights.
    // Standard: K=white king-side rook (h1=7), Q=white queen-side rook (a1=0),
    //           k=black king-side rook (h8=63), q=black queen-side rook (a8=56).
    // We also push the king square itself (e1=4, e8=60) when either side has rights.
    if parts.len() > 2 && parts[2] != "-" {
        let white_king_sq = board.wk.trailing_zeros() as i32;
        let black_king_sq = board.bk.trailing_zeros() as i32;
        let mut white_added = false;
        let mut black_added = false;

        for ch in parts[2].chars() {
            match ch {
                'K' => { // white king-side: rook to the RIGHT of the white king
                    let rook_sq = (white_king_sq + 1 ..= 7)
                        .find(|&s| board.wr & (1u64 << s) != 0)
                        .expect("FEN 'K' castling but no white rook right of king");
                    board.castle_rights.push(rook_sq);
                    if !white_added { board.castle_rights.push(white_king_sq); white_added = true; }
                }
                'Q' => { // white queen-side: rook to the LEFT of the white king
                    let rook_sq = (0 .. white_king_sq).rev()
                        .find(|&s| board.wr & (1u64 << s) != 0)
                        .expect("FEN 'Q' castling but no white rook left of king");
                    board.castle_rights.push(rook_sq);
                    if !white_added { board.castle_rights.push(white_king_sq); white_added = true; }
                }
                'k' => { // black king-side
                    let rook_sq = (black_king_sq + 1 ..= 63)
                        .find(|&s| board.br & (1u64 << s) != 0)
                        .expect("FEN 'k' castling but no black rook right of king");
                    board.castle_rights.push(rook_sq);
                    if !black_added { board.castle_rights.push(black_king_sq); black_added = true; }
                }
                'q' => { // black queen-side
                    let rook_sq = (56 .. black_king_sq).rev()
                        .find(|&s| board.br & (1u64 << s) != 0)
                        .expect("FEN 'q' castling but no black rook left of king");
                    board.castle_rights.push(rook_sq);
                    if !black_added { board.castle_rights.push(black_king_sq); black_added = true; }
                }
                _ => panic!("Unknown castling char in FEN: '{}'", ch),
            }
        }
    }

    // ── 4. En passant ───────────────────────────────────────────────
    if parts.len() > 3 && parts[3] != "-" {
        let ep_chars: Vec<char> = parts[3].chars().collect();
        assert!(ep_chars.len() == 2, "Bad en-passant field: '{}'", parts[3]);
        let ep_file = letter_to_x(ep_chars[0]);
        let ep_rank = ep_chars[1] as i32 - '1' as i32;
        board.en_passant = ep_rank * 8 + ep_file;
    }

    // ── 5. Half-move clock ──────────────────────────────────────────
    if parts.len() > 4 {
        board.counter = parts[4].parse::<i32>().unwrap_or(0);
    }

    // fullmove number (parts[5]) is ignored — your engine doesn't track it separately.

    board
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
    
    let castle = piece.abs() == 6 && self.piece_at(new_pos).abs() == 4 && piece * self.piece_at(new_pos) > 0;
        
    if piece.abs() == 6 || piece.abs() == 4 {self.castle_rights.retain(|&p| { p != pos} );}
    

    //is en passant check
    let en_pas = self.piece_at(pos).abs() == 1 && (pos % 8 != new_pos % 8) && self.piece_at(new_pos) == 0;
    //println!("EN PASSANT: {en_pas} | {new_pos}");
    //if en passant, remove piece at same quotient but +- 1 mod
    if en_pas{
        let remove_pos = (pos / 8) * 8 + (new_pos % 8);
        let pawns = match self.to_move{
            1 => &mut self.bp,
            -1 => &mut self.wp,
            _ => panic!("Invalid color"),
        };
        
        *pawns &= !(1u64 << remove_pos as u32);
    }
    //creates en passant option
    self.en_passant = -1;
    if self.piece_at(pos).abs() == 1{

        let (major,minor) = (std::cmp::max(pos, new_pos),std::cmp::min(pos, new_pos));
        if (major / 8) - (minor / 8) == 2{
            self.en_passant = minor + 8;
        }
    }

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



    pub fn get_fen(&self) -> String{
        //lowercase is black, uppercase is white
        //empty squares are numbers denoting empty len
        let mut buf = String::new();
        let mut empty_counter = 0;
        //go from topleft to botright
        for y in (0..8).rev(){
            for x in 0..8 {
            let i = y*8 + x;
            if i % 8 == 0 && i != 56 {
                if empty_counter != 0 {
                    buf.push_str( &format!("{}",empty_counter));
                    empty_counter = 0;
                }
                buf.push('/');
            }
            let p = self.piece_at(i);
            let initial = match p {
                1 => 'P',
                2 => 'N',
                3 => 'B',
                4 => 'R',
                5 => 'Q',
                6 => 'K',
                -1 => 'p',
                -2 => 'n',
                -3 => 'b',
                -4 => 'r',
                -5 => 'q',
                -6 => 'k',
                0 => 'e',
                _ => panic!("Invalid piece code"),
            };
            if initial == 'e' { empty_counter += 1; }  
            else {
                
                if empty_counter != 0 {
                    buf.push_str( &format!("{}",empty_counter));
                    empty_counter = 0;
                }
                
                buf.push(initial);

            }
        }   
        }
        if empty_counter != 0{ buf.push_str( &format!("{}",empty_counter)); }
       
        //to move
        let player = match self.to_move{
            1 => 'w',
            -1 => 'b',
            _ => panic!("Invalid color"),
        };
        
        buf.push_str(&format!(" {} ",player));
        //castle rights 
        //order: KQkq
        let (white_king,black_king) = (self.wk.trailing_zeros() as i32, self.bk.trailing_zeros() as i32);
        let (white_king_castle,black_king_castle) = (self.castle_rights.contains(&white_king),self.castle_rights.contains(&black_king));
        let mut space_flag = false;
        if black_king_castle{
        let black_kside = self.castle_rights.iter().any(|r| {
            (r / 8 == black_king / 8) && r > &black_king
        }); 
        let black_qside = self.castle_rights.iter().any(|r| {
            (r / 8 == black_king / 8) && r < &black_king
        });

        if black_kside {buf.push('K');space_flag=true;}//space flag
        if black_qside {buf.push('Q');space_flag=true;}//space flag
        }

        if white_king_castle{
        let white_kside = self.castle_rights.iter().any(|r| {
            (r / 8 == white_king / 8) && r > &white_king
        }); 
        let white_qside = self.castle_rights.iter().any(|r| {
            (r / 8 == white_king / 8) && r < &white_king
        
        });
        if white_kside {buf.push('k');space_flag=true;}//space flag
        if white_qside {buf.push('q');space_flag=true;}//space flag
        }

        if space_flag { buf.push(' ');} 
        //en passant if there is
        if self.en_passant != -1 {

            let letter = self.pos_to_letter(self.en_passant);
            let y = (self.en_passant/8) + 1;

            buf.push_str(&format!("{letter}{y} "));
        }

        else{buf.push_str("- ");}

        //halfmoves and moves
        buf.push_str(&format!("{} {}",self.counter,(self.counter/2) + 1));
        buf
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
