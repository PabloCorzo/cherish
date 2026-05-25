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
    to_move: bool,
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
            to_move:true,
        }

    }
}
