use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and",    TokenType::And);
        m.insert("class",  TokenType::Class);
        m.insert("else",   TokenType::Else);
        m.insert("false",  TokenType::False);
        m.insert("for",    TokenType::For);
        m.insert("fun",    TokenType::Fun);
        m.insert("if",     TokenType::If);
        m.insert("nil",    TokenType::Nil);
        m.insert("or",     TokenType::Or);
        m.insert("print",  TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super",  TokenType::Super);
        m.insert("this",   TokenType::This);
        m.insert("true",   TokenType::True);
        m.insert("var",    TokenType::Var);
        m.insert("while",  TokenType::While);
        m
    };
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
