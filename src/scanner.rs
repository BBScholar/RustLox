
use crate::token::*;
use crate::error_handling::*;

use std::collections::HashMap;
use std::vec::Vec;
use std::iter::FromIterator;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
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

#[derive(Debug)]
pub struct Scanner {
    source:  String,
    tokens:  Vec<Token>,
    chars :  Vec<char>,
    start :  u64,
    current: u64,
    line:    u64,
}

impl Scanner {

    pub fn new(source: String) -> Self {
        let temp_c: Vec<char> = source.chars().collect();
        Scanner {
            source,
            tokens:  Vec::new(),
            chars:       temp_c,
            start:            0,
            current:          0,
            line:             1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EndOfFile, "".to_string(), self.line, LiteralType::None()));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => { self.add_token(TokenType::LeftParen   ) }
            ')' => { self.add_token(TokenType::RightParen  ) }
            '{' => { self.add_token(TokenType::LeftBrace   ) }
            '}' => { self.add_token(TokenType::RightBrace  ) }
            ',' => { self.add_token(TokenType::Comma       ) }
            '.' => { self.add_token(TokenType::Dot         ) }
            '-' => { self.add_token(TokenType::Minus       ) }
            '+' => { self.add_token(TokenType::Plus        ) }
            ';' => { self.add_token(TokenType::Semicolon   ) }
            '*' => { self.add_token(TokenType::Star        ) }
            '!' => {
                let t = if self.mtch('=') { TokenType::BangEqual    } else { TokenType::Bang   };
                self.add_token(t);
            }
            '=' => {
                let t = if self.mtch('=') { TokenType::EqualEqual   } else { TokenType::Equal  };
                self.add_token(t)
            }
            '<' => {
                let t = if self.mtch('=') { TokenType::LessEqual    } else { TokenType::Less   };
                self.add_token(t);
            }
            '>' => {
                let t = if self.mtch('=') { TokenType::GreaterEqual } else { TokenType::Greater};
                self.add_token(t);
            }
            '/' => {
                if self.mtch('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                    self.line += 1;
                } else {
                    self.add_token( TokenType::Slash );
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => { self.line += 1; }
            '"' => { self.determine_string_literal() }
            'o' => {
                if self.peek() == 'r' {
                    self.add_token(TokenType::Or);
                }
            }
            _ =>  {
                if Self::is_digit(c) {
                    self.determine_number();
                } else if Self::is_alpha(c) {
                    self.determine_identifier();
                } else {
                    unsafe { error(self.line, "Unexpected character.") }
                }
            }
        }
    }

    fn advance(&mut self) -> char  {
        self.current += 1;
        self.chars[(self.current - 1) as usize]
    }

    fn mtch(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.chars[self.current as usize] != expected { return false; }
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() { return '\0'; }
        return self.chars[self.current as usize]
    }

    fn peek_next(&mut self) -> char {
        if (self.current + 1) >= self.source.len() as u64 { return '\0' }
        self.chars[(self.current + 1) as usize]
    }

    fn determine_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }
        if self.is_at_end() {
            unsafe {
                error(self.line, "Unterminated string.");
            }
            return;
        }

        self.advance();
        let ustart = (self.start + 1) as usize;
        let ucurrent = (self.current - 1) as usize;
        let value = String::from_iter(&self.chars[ustart..ucurrent]);
        self.add_token_with_literal(TokenType::String, LiteralType::String(value));
    }

    fn determine_number(&mut self) {
        while Self::is_digit(self.peek())  { self.advance(); }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek())  { self.advance(); }
        }
        let ustart = self.start as usize;
        let ucurrent = self.current as usize;
        let string = String::from_iter(&self.chars[ustart..ucurrent]);
        let value: f64 = string.parse().unwrap();
        self.add_token_with_literal(TokenType::Number, LiteralType::Number(value as f64));
    }

    fn determine_identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) { self.advance(); }
        let text = String::from_iter(&self.chars[(self.start as usize)..(self.current as usize)]);
        let t: TokenType = match KEYWORDS.get(&text.as_str()) {
            Some(value) => value.clone(),
            None        => TokenType::Identifier
        };
        self.add_token(t);
    }

    #[inline]
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    #[inline]
    fn is_alpha(c: char) -> bool {
        (c  >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    #[inline]
    fn is_alpha_numeric(c: char) -> bool  {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as u64)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, LiteralType::None())
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: LiteralType) {
        let ustart = self.start as usize;
        let ucurrent = self.current as usize;
        let text = String::from_iter(&self.chars[ustart..ucurrent]);
        self.tokens.push(Token::new(token_type, text, self.line, literal))
    }

}