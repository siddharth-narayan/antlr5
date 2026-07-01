use std::{env, fs::read_to_string};

use crate::antlr::Lexer;

mod antlr;


fn main() -> Result<(), ()> {
    let path = "test.g4";
    let content = read_to_string(path).map_err(|e| {println!("{}", e); ()})?;

    println!("{}", content);

    let mut lexer = Lexer::new(content);
    
    loop {
        match lexer.next_token() {
            Ok(t) => println!("{:#?}", t),
            Err(e) => {
                println!("{:#?}", e);
                break
            }
        }
    }

    Ok(())
}
