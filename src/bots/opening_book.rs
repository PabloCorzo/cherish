use std::collections::HashMap;
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OpeningBook{

    book: HashMap<[i32;12],Vec<(i32,i32)>>,
}

impl OpeningBook{
    

    pub fn new() -> Self {
    if let Ok(bytes) = std::fs::read("opening_book.bin") {
        if let Ok(book) = bincode::deserialize(&bytes) {
            return OpeningBook { book };
        }
    }

    // File doesn't exist yet, start with empty book
    OpeningBook { book: HashMap::new() }
}

}

impl Drop for OpeningBook{

        fn drop(&mut self){
            let bytes = bincode::serialize(&self.book).expect("Could not serialize opening book.");
            std::fs::write("opening_book.bin", bytes).expect("Could not write opening book to file.");
        }
}
