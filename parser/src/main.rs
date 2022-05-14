use std::fs;
use std::env;
use std::io::stdin;
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

struct CStream {
    filename: String,
    line_num: i32,
    char_pos: i32,
    cur_char: usize,
    file_data: Vec<u8>,
    file_str: String
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

impl CStream {
    //read file to a vector and initialize position trackers
    fn new(s: &str) -> CStream {
        CStream {
            filename: s.to_string(),
            line_num: -1,
            char_pos: -1,
            cur_char: 0,
            file_data: fs::read(s).expect("Unable to read file"),
            file_str: fs::read_to_string(s).expect("Unable to read file")
        }
    }
    //check if we are at the end of the vector
    fn more_available(&mut self) -> bool {
        if usize::from(self.cur_char) == self.file_data.len()-2 {
            return false;
        }
        return true;
    }
    //return the current char 
    fn get_cur_char(&mut self) -> char {
        return self.file_data[self.cur_char] as char;
    }
    //return the next char
    fn get_next_char(&mut self) -> char {
        //if we're at the start of the file
        if self.line_num == -1 && self.char_pos == -1 {
            self.line_num = 0;
            self.char_pos = 0;
            return self.file_data[self.cur_char] as char;
        }
        self.cur_char += 1;
        //if we're at the end of a line
        if self.file_data[self.cur_char] as char == '\n' {
            self.char_pos = 0;
            self.line_num += 1;
            self.cur_char += 1;
        }
        //if no special case applies
        else {
            self.char_pos += 1;
        }
        return self.file_data[self.cur_char] as char;
    }
    //return the next char, without modifying position trackers
    fn peek_next_char(&mut self) -> char {
        if self.line_num == -1 && self.char_pos == -1 {
            return self.file_data[self.cur_char] as char;
        }

        if self.file_data[self.cur_char+1] as char == '\n' {
            return self.file_data[self.cur_char+2] as char;
        }
        else {
            return self.file_data[self.cur_char+1] as char;
        }
    }
    //return the kth ahead char, without modifying position trackers
    fn peek_ahead_char(&mut self, k: i32) -> char {
        let mut new_char_num = self.cur_char;
        let mut count = 0;
        //account for being at the start of the file
        if self.line_num == -1 && self.char_pos == -1 {
            count += 1;
        }
        while count <= k {
            if self.file_data[new_char_num] as char == '\n' {
                new_char_num += 1;
            }
            else {
                new_char_num += 1;
                count += 1;
            }
        }
        if self.file_data[new_char_num] as char == '\n' {
            return self.file_data[new_char_num+1] as char;
        }
        return self.file_data[new_char_num] as char;
    }
}

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
        let next_char = self.cstrm.get_next_char();
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
    //fn check Keyword
    //fn check Identifier
}

fn main() {
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    let mut s = Scanner::new(&args[1]);
    for i in 0..150{
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
