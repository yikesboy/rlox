use crate::{parser::Parser, precedence::Precedence, token::TokenType, vm::InterpreterError};

pub type ParseFn = fn(&mut Parser) -> Result<(), InterpreterError>;

pub struct ParserRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

impl ParserRule {
    pub fn get_rule(token: TokenType) -> ParserRule {
        match token {
            TokenType::LeftParen => ParserRule {
                prefix: Some(Parser::grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightParen => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LeftBrace => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightBrace => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Comma => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Dot => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => ParserRule {
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParserRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Semicolon => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Slash => ParserRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Star => ParserRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Bang => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::BangEqual => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Equal => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EqualEqual => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Greater => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::GreaterEqual => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Less => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LessEqual => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Identifier => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::String => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Number => ParserRule {
                prefix: Some(Parser::number),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::And => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Class => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Else => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::False => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::For => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Fun => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::If => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Nil => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Or => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Print => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Return => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Super => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::This => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::True => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Var => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::While => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Eof => ParserRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }
}
