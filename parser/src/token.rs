#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]

pub enum TokenType
{
    IntConstant,
    FloatConstant,
    Keyword,
    Identifier,
    Operator,
    Invalid
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TokenType::IntConstant => "int constant",
            TokenType::FloatConstant => "float constant",
            TokenType::Keyword => "keyword",
            TokenType::Identifier => "identifier",
            TokenType::Operator => "operator",
            TokenType::Invalid => "invalid"
        }
    }
}

#[derive(Clone)]

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