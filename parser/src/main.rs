use std::fs::File;
use std::env;
use std::io::stdin;
use std::io::prelude::*;

mod cstream;
mod token;
mod scanner;
mod parser;
mod create_xhtml;
use crate::cstream::CStream;
use crate::token::*;
use crate::scanner::Scanner;
use crate::parser::*;
use crate::create_xhtml::create;

fn main() {
    //initialize all_tokens vector
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    
    //print out file to screen
    let mut s = Scanner::new(&args[1]);
    println!("{}", s.cstrm.file_str);

    //read in and store tokens into all_tokens vector
    while s.not_eof() {
        all_tokens.push(s.get_next_token());
    }
    //end of getting tokens, all tokens should now be stored in all_tokens
    
    //print out collected tokens
    let mut n=0;
    for token in all_tokens.iter()
    {
        println!("Token {} = {}\nToken Type: {:?}\n",n,token.text,token.token_type);
        n+=1;
    }

    //parse the vector of tokens
    let mut x_parser = Parser::new(&all_tokens);
    let result = x_parser.program();
    
    //if syntactically correct, create the xhtml file
    match result {
        Ok(_) => (create(all_tokens)), //create xhtml
        Err(e) => println!("{}", e) //throw the custom err
    }
}
