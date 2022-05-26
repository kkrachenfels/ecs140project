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
        println!("In program");
        
        //check second token
        if self.tokens[1].text != "main".to_string() {
            //descend into declaration 
            //loop
            let mut declaration_ret = self.declaration()?;
            while declaration_ret == 0 {
                declaration_ret = self.declaration()?;
            }
        }

        //descend into main_declaration
        let main_declar_ret = self.main_declaration()?;

        let mut funcdef_ret = self.declaration()?;
        while funcdef_ret == 0 {
            funcdef_ret = self.declaration()?;
        }

        //if no parse errors
        println!("Input program is syntactically correct");
        Ok(0)
    }

    //Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn declaration(&mut self) -> Result<i32, ParseError> {
        println!("In declaration");

        let declar_ret = self.declaration_type()?;

        //Ok(0) => 0
        //error(msg)

        //TO-DO: check that either one is Ok(0) (can't use question mark)
        let var_dec = self.variable_declaration();
        let func_dec = self.function_declaration();

        /*if var_dec == 0 || func_dec == 0 {
            Ok(0)
        }
        else {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration".to_string()});
        }*/

        //no errors
        Ok(0)
    }

    //MainDeclaration := void main ( ) Block
    pub fn main_declaration(&mut self) -> Result<i32, ParseError> {
        println!("In main declaration");

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
        self.inc_token();


        //if no errors in "void main ( )"
        let block_ret = self.block()?;

        //no errors
        Ok(0)
    }

    //FunctionDefinition := DeclarationType ParameterBlock Block
    pub fn function_definition(&mut self) -> Result<i32, ParseError> {
        println!("In function definition");

        self.declaration_type()?;

        self.parameter_block()?;

        self.block()?;

        //no errors
        Ok(0)
    }

    //DeclarationType := DataType Identifier
    pub fn declaration_type(&mut self) -> Result<i32, ParseError> {
        println!("In declaration type");

        let datatype_ret = self.data_type()?;
        self.inc_token();

        if self.tokens[self.t_num].token_type != TokenType::Identifier {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "DeclarationType := DataType Identifier".to_string()});
        }
        self.inc_token();

        //no errors
        Ok(0)
    }

    //VariableDeclaration := [= Constant] ;
    pub fn variable_declaration(&mut self) -> Result<i32, ParseError> {
        println!("In variable declaration");

        if self.tokens[self.t_num].text == "=".to_string() {
            self.inc_token();
            self.constant()?;
        }

        if self.tokens[self.t_num].text != ";".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "VariableDeclaration := [= Constant] ;".to_string()});
        }

        //no errors
        Ok(0)
    }

    //FunctionDeclaration := ParameterBlock ;
    pub fn function_declaration(&mut self) -> Result<i32, ParseError> {
        println!("In function_declaration");  

        self.parameter_block()?;
        self.inc_token();
        if self.tokens[self.t_num].text != ";".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "FunctionDeclaration := ParameterBlock ;".to_string()});
        }

        //no errors
        self.inc_token();
        Ok(0)
    }

    //Block := { {Declaration} {Statement} {FunctionDefinition} }
    pub fn block(&mut self) -> Result<i32, ParseError> {
        println!("In block");

        if self.tokens[self.t_num].text != "{".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
        }
        self.inc_token();

        //{declaration}
        let mut declaration_ret = self.declaration();
        while true {
            if let Ok(i) = declaration_ret {
                self.inc_token();
                declaration_ret = self.declaration();
            } else {
                break;
            }  
        }

        //{statement}
        let mut statement_ret = self.statement();
        while true {
            if let Ok(i) = statement_ret {
                self.inc_token();
                statement_ret = self.declaration();
            } else {
                break;
            }  
        }

        //{function definition}
        let mut funcdef_ret = self.function_definition();
        while true {
            if let Ok(i) = funcdef_ret {
                self.inc_token();
                funcdef_ret = self.declaration();
            } else {
                break;
            }  
        }

        Ok(0)

    }

    //ParameterBlock := ( [Parameter {, Parameter}] ) 
    pub fn parameter_block(&mut self) -> Result<i32, ParseError> {
        println!("In parameter block");

        if self.tokens[self.t_num].text != "(".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
        }
        self.inc_token();
        if self.tokens[self.t_num].text == ")".to_string() {
            self.inc_token();
            return Ok(0);
        }
        
        let mut param_ret = self.parameter()?;
        while param_ret == 0 {
            self.inc_token();
            if self.tokens[self.t_num].text != ",".to_string() {
                self.inc_token();
                param_ret = self.function_definition()?;
            }
        }

        if self.tokens[self.t_num].text != ")".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
        }

        //no errors
        self.inc_token();
        Ok(0)
    }

    //DataType := IntegerType | FloatType
    pub fn data_type(&mut self) -> Result<i32, ParseError> {
        println!("In data type");

        let int_ret = self.integer_type();
        let float_ret = self.float_type();

        if let Ok(i) = int_ret {
            self.inc_token();
            return Ok(0)
        } 
        if let Ok(i) = float_ret {
            self.inc_token();
            return Ok(0)
        } 
        //else {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "DataType := IntegerType | FloatType".to_string()});
        //}

        /*
        if (int_ret == 0 || float_ret == 0) {
            self.inc_token();
            Ok(0)
        }*/

        //no errors
        //Ok(0)
    }

    //Constant := IntConstant | FloatConstant
    pub fn constant(&mut self) -> Result<i32, ParseError> {
        println!("In constant");

        if self.tokens[self.t_num].token_type == TokenType::IntConstant || self.tokens[self.t_num].token_type == TokenType::FloatConstant {
            self.inc_token();
            return Ok(0);
        }
        //else errors
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "Constant := IntConstant | FloatConstant".to_string()});
    }

    /*Statement := Assignment | WhileLoop | IfStatement | 
        ReturnStatement | (Expression ;)*/
    pub fn statement(&mut self) -> Result<i32, ParseError> {
        println!("In statement");

        //no errors
        Ok(0)
    }

    //Parameter := DataType Identifier
    pub fn parameter(&mut self) -> Result<i32, ParseError> {
        println!("In parameter");

        let datatype_ret = self.data_type()?;
        self.inc_token();

        if self.tokens[self.t_num].token_type != TokenType::Identifier {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "DeclarationType := DataType Identifier".to_string()});
        }
        self.inc_token();

        //no errors
        Ok(0)
    }

    //IntegerType := [unsigned] (char | short | int | long)
    pub fn integer_type(&mut self) -> Result<i32, ParseError> {
        println!("In integer type");

        if self.tokens[self.t_num].text == "unsigned".to_string() {
            self.inc_token();
        }

        if self.tokens[self.t_num].text != "char".to_string() || self.tokens[self.t_num].text != "short".to_string() || self.tokens[self.t_num].text != "long".to_string() || self.tokens[self.t_num].text != "int".to_string()  {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "IntegerType := [unsigned] (char | short | int | long)".to_string()});
        }

        //no errors
        Ok(0)
    }

    //FloatType := float | double
    pub fn float_type(&mut self) -> Result<i32, ParseError> {
        println!("In float type");

        if self.tokens[self.t_num].text == "float".to_string() || self.tokens[self.t_num].text == "double".to_string() {
            return Ok(0)
        }

        //no errors
       // Ok(0)
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "FloatType := float | double".to_string()});
    }

    //Assignment := Identifier = {Identifier = } Expression;
    pub fn assignment(&mut self) -> Result<i32, ParseError> {
        println!("In assignment");

        //no errors
        Ok(0)
    }

    //WhileLoop := while ( Expression ) Block
    pub fn while_loop(&mut self) -> Result<i32, ParseError> {
        println!("In while loop");

        //no errors
        Ok(0)
    }

    //IfStatement := if ( Expression ) Block
    pub fn if_statement(&mut self) -> Result<i32, ParseError> {
        println!("In if statement");

        //no errors
        Ok(0)
    }

    //ReturnStatement := return Expression ;
    pub fn return_statement(&mut self) -> Result<i32, ParseError> {
        println!("In return statement");

        //no errors
        Ok(0)
    }

    //Expression := SimpleExpression [ RelationOperator SimpleExpression ]
    pub fn expression(&mut self) -> Result<i32, ParseError> {
        println!("In expression");

        //no errors
        Ok(0)
    }

    //SimpleExpression := Term { AddOperator Term }
    pub fn simple_expression(&mut self) -> Result<i32, ParseError> {
        println!("In simple expression");

        //no errors
        Ok(0)
    }

    //Term := Factor { MultOperator Factor }
    pub fn term(&mut self) -> Result<i32, ParseError> {
        println!("In term");

        //no errors
        Ok(0)
    }

    /*Factor := ( ( Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] ) */
    pub fn factor(&mut self) -> Result<i32, ParseError> {
        println!("In factor");

        //no errors
        Ok(0)
    }

    //RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )
    pub fn relation_operator(&mut self) -> Result<i32, ParseError> {
        println!("In relation operator");

        //no errors
        Ok(0)
    }

    //AddOperator := + | -
    pub fn add_operator(&mut self) -> Result<i32, ParseError> {
        println!("In add operator");

        //no errors
        Ok(0)
    }

    //MultOperator := * | /
    pub fn mult_operator(&mut self) -> Result<i32, ParseError> {
        println!("In mult operator");

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