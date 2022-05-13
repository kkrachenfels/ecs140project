use std::fs;
use std::env;
use std::io::stdin;
#[derive(Debug)]

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
//Todo: change this function later
fn token_creator(s: &str)->Token
{
    let mut t=Token{line_num:0,char_pos:0,text:s.to_string(),token_type:TokenType::Unassigned};
    if t.text=="0" || t.text=="1"
    {
        t.token_type=TokenType::Constant;
    }
    else if t.text=="a" || t.text=="b"||t.text=="c" || t.text=="d"
    {
        t.token_type=TokenType::Variable;
    }
    else if t.text==":="||t.text ==";"
    {
        t.token_type=TokenType::Special;
    }
    else
    {
        t.token_type=TokenType::Operator;
    }
    return t;
}

fn tokenize()
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
}

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

}

impl Scanner
{
    fn get_nxext_token(c:CStream)//->Token
    {
        let t= Token::new();
    }
}

fn main() {
    //let mut f = CStream::new("example1.x");
    //println!("{} {}", f.line_num, f.char_pos);
    //println!("{}", f.peek_next_char());
    let mut all_tokens:Vec<Token>=vec![];
    let args: Vec<String> = env::args().collect();
    let file= CStream::new(&args[1]);
    println!("{}",file.file_str);
}
