use crate::cstream::CStream;
use crate::token::*;

pub struct Scanner
{
    pub operator_list:Vec<char>,
    pub cstrm: CStream
}

impl Scanner
{
    pub fn new(filename: &str)->Scanner
    {
        Scanner {
            operator_list: ['(', ',', ')', '{', '}', '=', '<', '>','+', '-', '*', '/', ';'].to_vec(),
            cstrm: CStream::new(&filename)
        }
    }
    pub fn get_next_token(&mut self)->Token
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
    pub fn check_const(&mut self,mut elem:char)->Vec<char>
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
    pub fn check_op(&mut self, elem: char)->Vec<char>
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
    pub fn check_idk(&mut self,mut elem:char)->Vec<char>
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