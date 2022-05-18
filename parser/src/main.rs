use std::fs;
use std::env;
use std::io::stdin;

mod cstream;
mod token;
mod scanner;
use crate::cstream::CStream;
use crate::token::*;
use crate::scanner::Scanner;


/*fn tokenize()
{
    let mut token_str = String::new();
    let mut v : Vec<Token> = Vec::new();
    stdin().read_line(&mut token_str)
        .ok()
        .expect("Failed to read line");
    //println!("{}", token_str);
    for (i,c) in token_str.chars().enumerate()
    {
        if c ==':' && token_str.chars().nth(i+1).unwrap()=='='
        {
            let special_s=":=";
            v.push(token_creator(&special_s));
        }
        else if c =='=' && token_str.chars().nth(i-1).unwrap()==':'
        {
            continue;
        }
        else
        {
            v.push(token_creator(&c.to_string()));
        }
    }
    let mut n=0;
    for token in v.iter()
    {
        println!("Token {} = {}\nToken Type: {:?}\n",n,token.text,token.token_type);
        n+=1;
        if n==v.len()-1
        {
            break;
        }
    }
}*/



fn main() {
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    let mut s = Scanner::new(&args[1]);
    for i in 0..59{
        all_tokens.push(s.get_next_token())
    }
    let mut n=0;
    for token in all_tokens.iter()
    {
        println!("Token {} = {}\nToken Type: {:?}\n",n,token.text,token.token_type);
        n+=1;
        if n==all_tokens.len()-1
        {
            break;
        }
    }
}
