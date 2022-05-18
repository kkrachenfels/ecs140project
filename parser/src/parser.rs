extern crate custom_error;
use custom_error::custom_error;

use crate::token::*;


custom_error!{pub ParseError
    General{l: i32, c: i32, msg: String} = 
    "Error at Line {l} Character {c}. The syntax should be: {msg}."
}


pub struct Parser {
    pub tokens: Vec<Token>,
    pub t_num: usize,
    pub line_num: i32,
    pub char_pos: i32
}

impl Parser {
    pub fn new(tok_vec: &Vec<Token>) -> Parser {
        Parser {
            tokens: tok_vec.to_vec(),
            t_num: 0,
            line_num: tok_vec[0].line_num,
            char_pos: tok_vec[0].char_pos
        }
    }
    
    //Program := {Declaration} MainDeclaration {FunctionDefinition}
    pub fn program(&mut self) -> Result<i32, ParseError> {
        //descend into declaration 
        let declaration_ret = self.declaration()?;
        //descend into main_declaration
        let main_declar_ret = self.main_declaration()?;

        //if no parse errors
        println!("Input program is syntactically correct");
        Ok(0)
    }

    //Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn declaration(&mut self) -> Result<i32, ParseError> {
        let declart_ret = self.declaration_type()?;

        //no errors
        Ok(0)
    }

    //MainDeclaration := void main ( ) Block
    pub fn main_declaration(&mut self) -> Result<i32, ParseError> {
        //check for "void main ()" at the start of main declaration
        if self.tokens[self.t_num].text != "void".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "MainDeclaration := void main ( ) Block".to_string()});
        }
        self.inc_token();
        if self.tokens[self.t_num].text != "main".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "MainDeclaration := void main ( ) Block".to_string()});
        }
        self.inc_token();
        if self.tokens[self.t_num].text != "(".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "MainDeclaration := void main ( ) Block".to_string()});
        }
        self.inc_token();
        if self.tokens[self.t_num].text != ")".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "MainDeclaration := void main ( ) Block".to_string()});
        }

        //if no errors in "void main ( )"
        let block_ret = self.block()?;

        //no errors
        Ok(0)
    }

    //Block := { {Declaration} {Statement} {FunctionDefinition} }
    pub fn block(&mut self) -> Result<i32, ParseError> {

        //no errors
        Ok(0)
    }

    //DeclarationType := DataType Identifier
    pub fn declaration_type(&mut self) -> Result<i32, ParseError> {
    
        //no errors
        Ok(0)
    }

    //since we'll be iterating through the vector a lot
    pub fn inc_token(&mut self) {
        self.t_num += 1;
        self.line_num = self.tokens[self.t_num].line_num;
        self.char_pos = self.tokens[self.t_num].char_pos;
    }
}