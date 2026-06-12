use std::collections::HashMap;
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct OpeningBook{

    pub book: HashMap<[u64;12],Vec<(i32,i32,i32)>>,
}

impl OpeningBook{
    

    pub fn new() -> Self {
    if let Ok(bytes) = std::fs::read("opening_book.bin") {
        if let Ok(book) = bincode::deserialize(&bytes) {
            // println!("I COULD SERIALIZE YIPPEEEEE: \n {:?}",book);
            
            return OpeningBook { book };
        }
    }

    // File doesn't exist yet, start with empty book
    OpeningBook { book: HashMap::new() }
    //panic so it does not create a new one in case it overwrites current one
    // panic!("Could not load opening_book.bin");
    }

}

impl Drop for OpeningBook{

        fn drop(&mut self){
            let bytes = bincode::serialize(&self.book).expect("Could not serialize opening book.");
            std::fs::write("opening_book.bin", bytes).expect("Could not write opening book to file.");
        }
}
