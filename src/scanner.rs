
use crate::token::*;
use crate::error_handling::*;

use std::vec::Vec;
use std::iter::FromIterator;


#[derive(Debug)]
pub struct Scanner {
    source:  String,
    tokens:  Vec<Token>,
    start:   u64,
    current: u64,
    line:    u64,
}

impl Scanner {

    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens:  Vec::new(),
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
             _ =>  { unsafe { error(self.line,"Unexpected character.") } }
        }
    }

    fn advance(&mut self) -> char  {
        self.current += 1;
        let chars: Vec<char> = self.source.chars().collect();
        chars[(self.current - 1) as usize]
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as u64)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, LiteralType::None())
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: LiteralType) {
        let chars: Vec<char> = self.source.chars().collect();
        let ustart = self.start as usize;
        let ucurrent = self.current as usize;
        let text = String::from_iter(&chars[ustart..ucurrent]);
        self.tokens.push(Token::new(token_type, text, self.line, literal))
    }



}