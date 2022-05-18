use std::fs;
use std::env;
use std::io::stdin;

mod cstream;
use crate::cstream::CStream;

#[derive(Debug)]
#[derive(Clone, Copy)]

enum TokenType
{
    IntConstant,
    FloatConstant,
    Keyword,
    Identifier,
    Operator,
    Invalid
}

struct Token
{
    text:String,
    token_type:TokenType,
    line_num:i32,
    char_pos:i32
}

impl Token {
    fn new()->Token{
        Token{
            text : String::new(),
            token_type:TokenType::Invalid,
            line_num:0,
            char_pos:0
        }
    }
}


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


struct Scanner
{
    operator_list:Vec<char>,
    cstrm: CStream
}

impl Scanner
{
    fn new(filename: &str)->Scanner
    {
        Scanner {
            operator_list: ['(', ',', ')', '{', '}', '=', '<', '>','+', '-', '*', '/', ';'].to_vec(),
            cstrm: CStream::new(&filename)
        }
    }
    fn get_next_token(&mut self)->Token
    {
        let mut t= Token::new();
        let mut next_char = self.cstrm.get_next_char();
        while next_char==' '||next_char=='\n'
        {
            next_char = self.cstrm.get_next_char();
        }
        t.line_num = self.cstrm.line_num;
        t.char_pos = self.cstrm.char_pos;
        let mut token_text = self.check_op(next_char);
        if token_text.len() > 0 {
            t.token_type = TokenType::Operator;
            t.text = token_text.iter().collect(); //convert vec<char> to string and assign to t.text
            return t
        }
        token_text = self.check_const(next_char);
        if token_text.len() > 0
        {
            if token_text.contains(&'.') {
                t.token_type = TokenType::FloatConstant;
            }
            else {
                t.token_type = TokenType::IntConstant;
            }
            t.text = token_text.iter().collect(); //convert vec<char> to string and assign to t.text
            return t
        }
        token_text = self.check_idk(next_char);
        if token_text.len() > 0 {
            t.text = token_text.iter().collect();
            let keywords: Vec<String> = ["int".to_string(), "char".to_string(), "unsigned".to_string(),
                "short".to_string(), "long".to_string(), "float".to_string(), "double".to_string(),
                "while".to_string(), "if".to_string(), "main".to_string(), "void".to_string(),"return".to_string()].to_vec();
            for keyword in keywords {
                if t.text.eq(&keyword) {
                    t.token_type = TokenType::Keyword;
                    return t
                }
            }
            t.token_type = TokenType::Identifier;

            return t
        }
        // call checking functions if none are true then assign TokenType as invalid
        return t
    }
    fn check_const(&mut self,mut elem:char)->Vec<char>
    {
        let mut token_text: Vec<char> = vec![];
        // decimal point flag set to 1 if decimal point already encountered
        let mut dp_flag=0;
        if elem.is_digit(10) || (elem=='-' && self.cstrm.peek_next_char().is_digit(10))
        {
            token_text.push(elem);
            while self.cstrm.peek_next_char().is_digit(10) || (self.cstrm.peek_next_char()=='.' && dp_flag==0)
            {
                elem = self.cstrm.get_next_char();
                if elem == '.'
                {
                    dp_flag=1;
                }
                token_text.push(elem);
            }
        }
        return token_text
    }
    fn check_op(&mut self, elem: char)->Vec<char>
    {
        let mut token_text: Vec<char> = vec![];
        if (elem == '=' || elem=='<'||elem=='>'||elem=='!') && (self.cstrm.peek_next_char()=='=')
        {
            token_text.push(elem);
            token_text.push('=');
            return token_text
        }
        for i in 0..self.operator_list.len()
        {
            if self.operator_list[i] == elem
            {
                token_text.push(elem);
                return token_text
            }
        }
        return token_text
    }
    fn check_idk(&mut self,mut elem:char)->Vec<char>
    {
        let mut token_text: Vec<char> = vec![];
        if elem.is_alphabetic()||elem=='_'
        {
            token_text.push(elem);
            while self.cstrm.peek_next_char().is_alphanumeric() || self.cstrm.peek_next_char()=='_'
            {
                elem = self.cstrm.get_next_char();
                token_text.push(elem);
            }
        }
        return token_text
    }
}

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
