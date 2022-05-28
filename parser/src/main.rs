use std::env;

mod cstream;
mod token;
mod scanner;
mod parser;
use crate::token::*;
use crate::scanner::Scanner;
use crate::parser::*;



fn main() {
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    let mut s = Scanner::new(&args[1]);

    //read in all tokens
    while s.not_eof() {
        all_tokens.push(s.get_next_token());
    }

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
    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
}
