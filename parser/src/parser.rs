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
        
        //check for zero or more declarations
        while self.tokens[self.t_num].text != "void".to_string() {
            self.declaration()?;
        }

        //descend into main_declaration
        self.main_declaration()?;

        //check for zero or more function definitions
        let mut funcdef_ret = self.function_definition();
        
        if let Ok(_i) = funcdef_ret {
            funcdef_ret = self.function_definition();
        }

        //if no parse errors
        println!("Input program is syntactically correct");
        Ok(0)
    }

    //Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn declaration(&mut self) -> Result<i32, ParseError> {
        println!("In declaration");

        //check mandatory declaration type
        self.declaration_type()?;

        let func_dec = self.function_declaration();
        let var_dec = self.variable_declaration();

        //if there is either a variable or function declaration
        if let Ok(_i) = func_dec {
            return Ok(0);
        }
        if let Ok(_i) = var_dec {
            return Ok(0)
        } 

        //else, error
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)".to_string()}); 

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
        self.block()?;

        println!("LEAVING MAIN DECLARATION");
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
        println!("RETURNING FROM FUNCTION DEFINITION!");
        Ok(0)
    }

    //DeclarationType := DataType Identifier
    pub fn declaration_type(&mut self) -> Result<i32, ParseError> {
        println!("In declaration type");

        self.data_type()?;

        if self.tokens[self.t_num].token_type == TokenType::Identifier {
            self.inc_token();
            return Ok(0)
        }
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "DeclarationType := DataType Identifier".to_string()});
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
        self.inc_token();
        println!("returned from variable declaration!");
        Ok(0)
    }

    //FunctionDeclaration := ParameterBlock ;
    pub fn function_declaration(&mut self) -> Result<i32, ParseError> {
        println!("In function_declaration");  

        self.parameter_block()?;
        if self.tokens[self.t_num].text != ";".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "FunctionDeclaration := ParameterBlock ;".to_string()});
        }

        //no errors
        self.inc_token();
        println!("returned from function declaration");
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
        //if block is empty
        if self.tokens[self.t_num].text == "}".to_string() {
            return Ok(0);
        }

        //else, there must be something inside the block
        let mut num_declarations = 0;
        let mut num_statements = 0;
        let mut num_funcdef = 0;

        //{declaration}
        let mut declaration_ret = self.declaration();
        loop {
            if let Ok(_i) = declaration_ret {
                //self.inc_token();
                num_declarations = num_declarations + 1;
                declaration_ret = self.declaration();
            } else {
                break;
            }  
        }
        //{statement}
        let mut statement_ret = self.statement();
        loop {
            if let Ok(_i) = statement_ret {
                //self.inc_token();
                num_statements = num_statements + 1;
                statement_ret = self.statement();
            } else {
                break;
            }  
        }
        //{function definition}
        let mut funcdef_ret = self.function_definition();
        loop {
            if let Ok(_i) = funcdef_ret {
                //self.inc_token();
                num_funcdef = num_funcdef + 1;
                funcdef_ret = self.function_definition();
            } else {
                break;
            }  
        }

        //if all three checks threw errors 
        if num_declarations == 0 && num_statements == 0 && num_funcdef == 0 {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
        }

        //if no closing brace
        if self.tokens[self.t_num].text != "}".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string()});
        }
        
        println!("RETURNING FROM BLOCK");
        self.inc_token();
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
        //if empty parameter block
        if self.tokens[self.t_num].text == ")".to_string() {
            self.inc_token();
            return Ok(0);
        }
        
        let mut param_ret = self.parameter()?;
        while param_ret == 0 {
            if self.tokens[self.t_num].text == ",".to_string() {
                self.inc_token();
                param_ret = self.parameter()?;
            }
            else if self.tokens[self.t_num].text == ")".to_string() {
                self.inc_token();
                println!("reached end of parameter block");
                return Ok(0);
            }
            else {
                return Err(ParseError::General{l: self.line_num, c: self.char_pos, msg: "ParameterBlock := ( [Parameter {, Parameter}] )".to_string()});
            }
        }

        println!("reached end of parameter block");
        self.inc_token();
        Ok(0) 
    }

    //DataType := IntegerType | FloatType
    pub fn data_type(&mut self) -> Result<i32, ParseError> {
        println!("In data type");

        let int_ret = self.integer_type();
        let float_ret = self.float_type();

        //if either integer type or float type found
        if let Ok(_i) = int_ret {
            self.inc_token();
            return Ok(0)
        } 
        if let Ok(_i) = float_ret {
            self.inc_token();
            return Ok(0)
        } 

        //println!("didn't find int or float type");
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "DataType := IntegerType | FloatType".to_string()});
    }

    //Constant := IntConstant | FloatConstant
    pub fn constant(&mut self) -> Result<i32, ParseError> {
        println!("In constant");

        if self.tokens[self.t_num].token_type == TokenType::IntConstant || self.tokens[self.t_num].token_type == TokenType::FloatConstant {
            self.inc_token();
            return Ok(0);
        }

        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "Constant := IntConstant | FloatConstant".to_string()});
    }

    /*Statement := Assignment | WhileLoop | IfStatement | 
        ReturnStatement | (Expression ;)*/
    pub fn statement(&mut self) -> Result<i32, ParseError> {
        println!("In statement");
        
        let while_ret = self.while_loop();
        let if_ret = self.if_statement();
        let ret_ret = self.return_statement();
        let assign_ret = self.assignment();
        let exp_ret = self.expression();

        if let Ok(_i) = assign_ret {
            println!("leaving statement");
            return Ok(0)
        } 
        if let Ok(_i) = while_ret {
            println!("leaving statement");
            return Ok(0)
        } 
        if let Ok(_i) = if_ret {
            println!("leaving statement");
            return Ok(0)
        } 
        if let Ok(_i) = ret_ret {
            println!("leaving statement");
            return Ok(0)
        }
        if let Ok(_i) = exp_ret {
            if self.tokens[self.t_num].text == ";".to_string() {
                self.inc_token();
                return Ok(0)
            }
        }

        println!("leaving statement");
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string()});
    }

    //Parameter := DataType Identifier
    pub fn parameter(&mut self) -> Result<i32, ParseError> {
        println!("In parameter");

        self.data_type()?;

        if self.tokens[self.t_num].token_type != TokenType::Identifier {
            println!("found identifier!");
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "DeclarationType := DataType Identifier".to_string()});
        }

        //no errors
        println!("Leaving parameter");
        self.inc_token();
        Ok(0)
    }

    //IntegerType := [unsigned] (char | short | int | long)
    pub fn integer_type(&mut self) -> Result<i32, ParseError> {
        println!("In integer type");

        if self.tokens[self.t_num].text == "unsigned".to_string() {
            self.inc_token();
        }

        if self.tokens[self.t_num].text == "char".to_string() || self.tokens[self.t_num].text == "short".to_string() || self.tokens[self.t_num].text == "long".to_string() || self.tokens[self.t_num].text == "int".to_string()  {
            return Ok(0)
        }

        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "IntegerType := [unsigned] (char | short | int | long)".to_string()});
    }

    //FloatType := float | double
    pub fn float_type(&mut self) -> Result<i32, ParseError> {
        println!("In float type");

        if self.tokens[self.t_num].text == "float".to_string() || self.tokens[self.t_num].text == "double".to_string() {
            //self.inc_token();
            return Ok(0)
        }

        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "FloatType := float | double".to_string()});
    }

    //Assignment := Identifier = {Identifier = } Expression;
    pub fn assignment(&mut self) -> Result<i32, ParseError> {
        println!("In assignment");

        if self.tokens[self.t_num].token_type != TokenType::Identifier || self.tokens[self.t_num+1].text != "=".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Assignment := Identifier = {Identifier = } Expression;".to_string()});
        }
        self.inc_token();
        self.inc_token();

        while self.tokens[self.t_num].token_type == TokenType::Identifier {
            //println!("checking assignment while loop");
            //self.inc_token();
            if self.tokens[self.t_num+1].text != "=" {
                break;
            }
            self.inc_token();
            self.inc_token();
        }

        //check for expression
        self.expression()?;

        //println!("RESUMING ASSIGNMENT");

        if self.tokens[self.t_num].text != ";".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "Assignment := Identifier = {Identifier = } Expression;".to_string()});
        }

        //no errors
        println!("leaving assigment");
        self.inc_token();
        Ok(0)
    }

    //WhileLoop := while ( Expression ) Block
    pub fn while_loop(&mut self) -> Result<i32, ParseError> {
        println!("In while loop");

        if self.tokens[self.t_num].text != "while".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "WhileLoop := while ( Expression ) Block".to_string()});
        }
        self.inc_token();
        if self.tokens[self.t_num].text != "(".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "WhileLoop := while ( Expression ) Block".to_string()});
        }
        self.inc_token();
        //println!("ABOUT TO CHECK EXPRESSION IN WHILE LOOP");
        self.expression()?;
        //println!("CAME BACK FROM EXPRESSION IN WHILE LOOP");
        if self.tokens[self.t_num].text != ")".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "WhileLoop := while ( Expression ) Block".to_string()});
        }
        self.inc_token();
        self.block()?;

        //no errors
        Ok(0)
    }

    //IfStatement := if ( Expression ) Block
    pub fn if_statement(&mut self) -> Result<i32, ParseError> {
        println!("In if statement");
        
        if self.tokens[self.t_num].text != "if".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "ReturnStatement := return Expression ;".to_string()});
        }

        Ok(0)
    }

    //ReturnStatement := return Expression ;
    pub fn return_statement(&mut self) -> Result<i32, ParseError> {
        println!("In return statement");

        if self.tokens[self.t_num].text != "return".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "ReturnStatement := return Expression ;".to_string()});
        }
        self.inc_token();
        self.expression()?;
        if self.tokens[self.t_num].text != ";".to_string() {
            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "ReturnStatement := return Expression ;".to_string()});
        }

        //no errors
        println!("FINISHED with return statement");
        self.inc_token();
        Ok(0)
    }

    //Expression := SimpleExpression [ RelationOperator SimpleExpression ]
    pub fn expression(&mut self) -> Result<i32, ParseError> {
        println!("In expression");

        self.simple_expression()?;

        let rel_op = self.relation_operator();

        if let Ok(_i) = rel_op {
            self.simple_expression()?;
        } 
        
        println!("leaving expression");
        return Ok(0);
    }

    //SimpleExpression := Term { AddOperator Term }
    pub fn simple_expression(&mut self) -> Result<i32, ParseError> {
        println!("In simple expression");

        self.term()?;

        let mut add_op = self.add_operator();
        loop {
            if let Ok(_i) = add_op {
                //println!("LOOPING IN SIMPLE EXPRESSION");
                self.term()?;
                add_op = self.add_operator();
            } else {
                break;
            }  
        }
        //no errors
        println!("leaving simple expression");
        Ok(0)
    }

    //Term := Factor { MultOperator Factor }
    pub fn term(&mut self) -> Result<i32, ParseError> {
        println!("In term");

        self.factor()?;

        let mut mul_op = self.mult_operator();
        loop {
            if let Ok(_i) = mul_op {
                self.factor()?;
                mul_op = self.mult_operator();
            } else {
                break;
            }
        }

        //no errors
        println!("leaving term");
        Ok(0)
    }

    /*Factor := ( ( Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] ) */
    pub fn factor(&mut self) -> Result<i32, ParseError> {
        println!("In factor");

        //if factor is an expression
        if self.tokens[self.t_num].text == "(".to_string() {
            self.inc_token();
            self.expression()?;
            if self.tokens[self.t_num].text != ")".to_string() {
                return Err(ParseError::General{l: self.line_num, c: self.char_pos, msg: "Factor := ( ( Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string()});
            } else {
                return Ok(0)
            }
        }
        //if factor is identifier
        if self.tokens[self.t_num].token_type == TokenType::Identifier {
            if self.tokens[self.t_num+1].text == "(".to_string() {
                self.inc_token();
                self.inc_token();
                if self.tokens[self.t_num].text == ")".to_string() {
                    return Ok(0)
                } else {
                    let mut exp_ret = self.expression()?;
                    while exp_ret == 0 {
                        //self.inc_token();
                        if self.tokens[self.t_num].text == ",".to_string() {
                            self.inc_token();
                            exp_ret = self.expression()?;
                        }
                        else if self.tokens[self.t_num].text == ")".to_string() {
                            self.inc_token();
                            //println!("reached end of expression block");
                            return Ok(0);
                        }
                        else {
                            return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                            msg: "Factor := ( ( Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string()});
                        }
                    }
                }
            } else {
                println!("found an identifier, leaving factor");
                self.inc_token();
                return Ok(0)
            }
        }
        //if we are here the factor has to be a constant
        self.constant()?;
        println!("leaving factor");

        //no errors
        Ok(0)
    }

    //RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )
    pub fn relation_operator(&mut self) -> Result<i32, ParseError> {
        println!("In relation operator");

        if self.tokens[self.t_num].text == "==".to_string() || self.tokens[self.t_num].text == "<".to_string() || self.tokens[self.t_num].text == ">".to_string() || self.tokens[self.t_num].text == "<=".to_string() || self.tokens[self.t_num].text == ">=".to_string() || self.tokens[self.t_num].text == "!=".to_string() {
            self.inc_token();
            return Ok(0)
        }
        println!("leaving rel op");
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
            msg: "RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )".to_string()});
        
    }

    //AddOperator := + | -
    pub fn add_operator(&mut self) -> Result<i32, ParseError> {
        println!("In add operator");
        if self.tokens[self.t_num].text == "+".to_string() || self.tokens[self.t_num].text == "-".to_string()  {
            self.inc_token();
            return Ok(0)
        }
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "AddOperator := + | -".to_string()});
    }

    //MultOperator := * | /
    pub fn mult_operator(&mut self) -> Result<i32, ParseError> {
        println!("In mult operator");
        if self.tokens[self.t_num].text == "*".to_string() || self.tokens[self.t_num].text == "/".to_string()  {
            self.inc_token();
            return Ok(0)
        }
        return Err(ParseError::General{l: self.line_num, c: self.char_pos, 
                msg: "MultOperator := * | /".to_string()});
    }

    //since we'll be iterating through the vector a lot
    pub fn inc_token(&mut self) {
        if self.t_num+1 != self.tokens.len() {
            self.t_num += 1;
            self.line_num = self.tokens[self.t_num].line_num;
            self.char_pos = self.tokens[self.t_num].char_pos;
            println!("On token: {}, type: {}", self.tokens[self.t_num].text, self.tokens[self.t_num].token_type.as_str()); 
        }
    }
}