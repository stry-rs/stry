use std::{iter::Peekable, str::CharIndices};

use super::Span;

#[derive(Debug)]
pub struct Token<'i> {
    pub span: Span<'i>,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // implementation
    Ident,

    // keywords
    Enum,
    Function,
    Service,
    Type,

    // symbols
    Minus,
    Comma,
    Colon,
    GreaterThan,
    LessThan,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,

    // whitespace
    Space,
    Tab,
    Newline,
    CarriageReturn,

    // other
    Error,
}

pub struct Lexer<'i> {
    input: &'i str,
    indices: Peekable<CharIndices<'i>>,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            indices: input.char_indices().peekable(),
            input,
        }
    }

    fn is_next_ident(&mut self) -> bool {
        self.indices
            .peek()
            .map(|(_, c)| match c {
                c if c.is_alphanumeric() => true,
                '_' => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = Token<'i>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.indices.next() {
            Some((i, c)) => {
                let (kind, range) = match c {
                    // symbols
                    '-' => (TokenKind::Minus, i..(i + 1)),
                    ',' => (TokenKind::Comma, i..(i + 1)),
                    ':' => (TokenKind::Colon, i..(i + 1)),
                    '>' => (TokenKind::GreaterThan, i..(i + 1)),
                    '<' => (TokenKind::LessThan, i..(i + 1)),
                    '{' => (TokenKind::OpenBrace, i..(i + 1)),
                    '}' => (TokenKind::CloseBrace, i..(i + 1)),
                    '[' => (TokenKind::OpenBracket, i..(i + 1)),
                    ']' => (TokenKind::CloseBracket, i..(i + 1)),
                    '(' => (TokenKind::OpenParen, i..(i + 1)),
                    ')' => (TokenKind::CloseParen, i..(i + 1)),

                    // whitespace
                    ' ' => (TokenKind::Space, i..(i + 1)),
                    '\t' => (TokenKind::Tab, i..(i + 1)),
                    '\n' => (TokenKind::Newline, i..i),
                    '\r' => (TokenKind::CarriageReturn, i..i),

                    // implementation / keywords
                    c if c.is_alphanumeric() || c == '_' => {
                        let (kind, range) = if self.is_next_ident() {
                            loop {
                                match self.indices.next() {
                                    Some((j, _)) => {
                                        if !self.is_next_ident() {
                                            break (TokenKind::Ident, i..(j + 1));
                                        }
                                    }
                                    None => {
                                        break (TokenKind::Ident, i..(i + 1));
                                    }
                                }
                            }
                        } else {
                            (TokenKind::Ident, i..(i + 1))
                        };

                        match &self.input[range.clone()] {
                            "enum" => (TokenKind::Enum, range),
                            "fn" => (TokenKind::Function, range),
                            "service" => (TokenKind::Service, range),
                            "type" => (TokenKind::Type, range),
                            _ => (kind, range),
                        }
                    }

                    // unknown
                    _ => (TokenKind::Error, i..(i + 1)),
                };

                Some(Token {
                    span: Span {
                        input: self.input,
                        range,
                    },
                    kind,
                })
            }
            None => None,
        }
    }
}
