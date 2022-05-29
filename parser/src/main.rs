use std::fs::File;
use std::env;

mod cstream;
mod token;
mod scanner;
mod parser;
mod create_xhtml;
use crate::token::*;
use crate::scanner::Scanner;
use crate::parser::*;
use crate::create_xhtml::create;

fn main() {
    //initialize all_tokens vector
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    
    //PART 1: print out file to screen
    let mut s = Scanner::new(&args[1]);
    println!("{}", s.cstrm.file_str);

    //PART 2: read in and store tokens into all_tokens vector
    while s.not_eof() {
        all_tokens.push(s.get_next_token());
    }
    //end of getting tokens, all tokens should now be stored in all_tokens
    
    //PART 3: parse the vector of tokens
    let mut x_parser = Parser::new(&all_tokens);
    let result = x_parser.program();
    
    //PART 4: if syntactically correct, create the xhtml file
    match result {
        Ok(_) => (create(all_tokens, &args[1])), //create xhtml
        Err(e) => println!("{}", e) //throw the custom err
    }
}
