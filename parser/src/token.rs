#[derive(Debug)]

pub enum TokenType
{
    IntConstant,
    FloatConstant,
    Keyword,
    Identifier,
    Operator,
    Invalid
}

pub struct Token
{
    pub text:String,
    pub token_type:TokenType,
    pub line_num:i32,
    pub char_pos:i32
}

impl Token {
    pub fn new()->Token{
        Token{
            text : String::new(),
            token_type:TokenType::Invalid,
            line_num:0,
            char_pos:0
        }
    }
}