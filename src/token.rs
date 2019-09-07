
#[allow(non_snake_case, dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {    
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual, 
    Equal, EqualEqual,
    Greater, GreaterEqual, 
    Less, LessEqual,
    
    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, 

    EndOfFile,
}

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    None(),
    Identifier(String),
    String(String),
    Number(f64)
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u64,
    literal: LiteralType
}


impl Token {

    pub fn new(token_type: TokenType, lexeme: String, line: u64, literal: LiteralType) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

}
