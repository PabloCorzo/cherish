use std::io;
use std::io::Write;

pub fn input_tui() -> String{                                                                                                                                                                            
    print!("Enter your move: ");                                                                                                                                                                     
    io::stdout().flush().unwrap();                                                                                                                                                                   
                                                                                                                                                                                                     
    let mut input = String::new();                                                                                                                                                                   
    io::stdin().read_line(&mut input).unwrap();                                                                                                                                                      
    let play = input.trim();

    play.into()                                                                                                                                                                                      
                                                                                                                                                                                                       
  }            


