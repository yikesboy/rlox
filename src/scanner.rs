use std::{error::Error, panic};

use crate::token::{Span, Token, TokenType};

#[derive(Debug)]
pub struct ScanError {
    pub kind: ScanErrorKind,
    pub span: Span,
    pub line: usize,
}

#[derive(Debug)]
pub enum ScanErrorKind {
    UnexpectedCharacter(char),
    UnterminatedString,
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 0,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, ScanError> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.create_token(TokenType::Eof);
        }

        let current = self.advance().expect("already checked is_at_end before");

        if Self::is_alpha(current) {
            return self.create_identifier();
        }

        if current.is_ascii_digit() {
            return self.create_number();
        }

        match current {
            '(' => self.create_token(TokenType::LeftParen),
            ')' => self.create_token(TokenType::RightParen),
            '{' => self.create_token(TokenType::LeftBrace),
            '}' => self.create_token(TokenType::RightBrace),
            ';' => self.create_token(TokenType::Semicolon),
            ',' => self.create_token(TokenType::Comma),
            '.' => self.create_token(TokenType::Dot),
            '-' => self.create_token(TokenType::Minus),
            '+' => self.create_token(TokenType::Plus),
            '/' => self.create_token(TokenType::Slash),
            '*' => self.create_token(TokenType::Star),
            '!' => {
                let tokentype = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.create_token(tokentype)
            }
            '=' => {
                let tokentype = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.create_token(tokentype)
            }
            '<' => {
                let tokentype = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.create_token(tokentype)
            }
            '>' => {
                let tokentype = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.create_token(tokentype)
            }
            '"' => self.create_string(),
            other => Err(self.error(ScanErrorKind::UnexpectedCharacter(other))),
        };

        return Err(self.error(ScanErrorKind::UnexpectedCharacter(' ')));
    }

    pub fn lexeme(&self, token: &Token) -> &str {
        &self.source[token.span.start..token.span.end]
    }

    fn create_identifier(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_some_and(Self::is_alpha_numeric) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let identifier = match Self::identifier_kind(text) {
            Some(kind) => self.create_token(kind),
            None => self.create_token(TokenType::Identifier),
        };

        identifier
    }

    fn identifier_kind(lexeme: &str) -> Option<TokenType> {
        match lexeme {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    fn create_number(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        self.create_token(TokenType::Number)
    }

    fn create_string(&mut self) -> Result<Token, ScanError> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(self.error(ScanErrorKind::UnterminatedString));
        }

        self.advance();
        self.create_token(TokenType::String)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') if self.peek_next() == Some('/') => {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            };
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.current..].chars();

        chars.next()?;
        chars.next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.advance();
        true
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.current += c.len_utf8();
        Some(c)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn create_token(&self, type_: TokenType) -> Result<Token, ScanError> {
        Ok(Token {
            type_,
            span: Span {
                start: self.start,
                end: self.current,
            },
            line: self.line,
        })
    }

    fn error(&self, kind: ScanErrorKind) -> ScanError {
        ScanError {
            kind,
            span: Span {
                start: self.start,
                end: self.current,
            },
            line: self.line,
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
