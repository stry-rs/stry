use std::ops::Range;

#[derive(PartialEq)]
pub struct Span<'i> {
    pub input: &'i str,
    pub range: Range<usize>,
}

impl<'i> std::fmt::Debug for Span<'i> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Span")
            .field("range", &self.range)
            .field("text", &&self.input[self.range.clone()])
            .finish()
    }
}

pub mod parser {
    use {
        super::{
            lexer::{Lexer, Token, TokenKind},
            Span,
        },
        std::iter::{Filter, Peekable},
    };

    macro_rules! take {
        ($lexer:ident, $errors:ident : [
            $(
                $( #[$attr:meta] )*
                $token:expr => |$got:ident| $code:block ,
            )+
        ]) => {
            match $lexer.next()? {
                $(
                    $( #[$attr] )*
                    $got if $got.kind == $token => { $code },
                )+
                Token { kind, span } => {
                    $errors.push(Error {
                        span,
                        got: kind,
                        expected: vec![
                            $( $token , )+
                        ],
                    });
                }
            }
        };
    }

    #[derive(Debug)]
    pub struct Error<'i> {
        pub span: Span<'i>,
        pub got: TokenKind,
        pub expected: Vec<TokenKind>,
    }

    pub struct FilteredLexer<'i> {
        lexer: Filter<Lexer<'i>, fn(&Token<'i>) -> bool>,
    }

    impl<'i> FilteredLexer<'i> {
        fn new(lexer: Lexer<'i>) -> Self {
            Self {
                lexer: lexer.filter(|token| {
                    !matches!(
                        token.kind,
                        TokenKind::Space
                            | TokenKind::Tab
                            | TokenKind::Newline
                            | TokenKind::CarriageReturn
                    )
                }),
            }
        }
    }

    impl<'i> Iterator for FilteredLexer<'i> {
        type Item = Token<'i>;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            self.lexer.next()
        }
    }

    pub trait Ast<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized;
    }

    pub struct Parser<'i> {
        lexer: Peekable<FilteredLexer<'i>>,
        errors: Vec<Error<'i>>,
    }

    impl<'i> Parser<'i> {
        pub fn new(lexer: Lexer<'i>) -> Self {
            Self {
                lexer: FilteredLexer::new(lexer).peekable(),
                errors: vec![],
            }
        }

        pub fn parse<A>(&mut self) -> Option<A>
        where
            A: Ast<'i>,
        {
            A::parse(&mut self.lexer, &mut self.errors)
        }
    }

    // TODO: finish ast
    #[derive(Debug, PartialEq)]
    pub struct File<'i> {
        pub stmts: Vec<Stmt<'i>>,
    }

    // TODO: finish ast
    #[derive(Debug, PartialEq)]
    pub enum Stmt<'i> {
        Enum {
            name: Ident<'i>,
            generics: Vec<Type<'i>>,
            variants: Vec<Ident<'i>>,
        },
        Service {
            name: Ident<'i>,
            functions: Vec<Function<'i>>,
        },
        Type {
            name: Ident<'i>,
            generics: Vec<Type<'i>>,
            fields: Vec<Field<'i>>,
        },
    }

    #[derive(Debug, PartialEq)]
    pub struct Field<'i> {
        pub name: Ident<'i>,
        pub typ: Type<'i>,
    }

    impl<'i> Ast<'i> for Field<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized,
        {
            take!(lexer, errors: [
                TokenKind::Ident => |token| {
                    take!(lexer, errors: [
                        TokenKind::Colon => |_token| {},
                    ]);

                    let typ: Type<'i> = Type::parse(lexer, errors)?;

                    return Some(Field {
                        name: Ident {
                            span: Span {
                                input: token.span.input,
                                range: token.span.range.clone(),
                            },
                        },
                        typ,
                    });
                },
            ]);

            None
        }
    }

    #[test]
    fn test_field_type_array() {
        let input = "array: [Int]";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Field {
                name: Ident {
                    span: Span { input, range: 0..5 },
                },
                typ: Type::Array {
                    typ: Box::new(Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 8..11
                            },
                        },
                    }),
                }
            }),
            parser.parse::<Field>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_field_type_generic() {
        let input = "generic: Edge <Int>";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Field {
                name: Ident {
                    span: Span { input, range: 0..7 },
                },
                typ: Type::Generic {
                    typ: Ident {
                        span: Span {
                            input,
                            range: 9..13
                        },
                    },
                    generics: vec![Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 15..18
                            },
                        },
                    }],
                }
            }),
            parser.parse::<Field>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_field_type_standard() {
        let input = "standard: Int";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Field {
                name: Ident {
                    span: Span { input, range: 0..8 },
                },
                typ: Type::Standard {
                    typ: Ident {
                        span: Span {
                            input,
                            range: 10..13
                        },
                    },
                }
            }),
            parser.parse::<Field>(),
            "{:?}",
            parser.errors,
        );
    }

    // TODO: finish ast
    #[derive(Debug, PartialEq)]
    pub struct Function<'i> {
        pub name: Ident<'i>,
        pub generics: Vec<Type<'i>>,
        pub fields: Vec<Field<'i>>,
        pub ret: Type<'i>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Type<'i> {
        Array {
            typ: Box<Type<'i>>,
        },
        Generic {
            typ: Ident<'i>,
            generics: Vec<Type<'i>>,
        },
        Standard {
            typ: Ident<'i>,
        },
    }

    impl<'i> Ast<'i> for Type<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized,
        {
            take!(lexer, errors: [
                // start of array variant
                TokenKind::OpenBracket => |_token| {
                    let typ: Type<'i> = Type::parse(lexer, errors)?;

                    take!(lexer, errors: [
                        TokenKind::CloseBracket => |_token| {},
                    ]);

                    return Some(Type::Array {
                        typ: Box::new(typ),
                    });
                },
                // start of generic and standard variants
                TokenKind::Ident => |token| {
                    match lexer.peek() {
                        Some(_token) if _token.kind == TokenKind::LessThan => {
                            take!(lexer, errors: [
                                TokenKind::LessThan => |token| {},
                            ]);

                            let mut generics = Vec::new();

                            loop {
                                let typ: Type<'i> = Type::parse(lexer, errors)?;

                                generics.push(typ);

                                match lexer.peek()? {
                                    token if token.kind == TokenKind::Comma => {}
                                    _ => break,
                                }
                            }

                            take!(lexer, errors: [
                                TokenKind::GreaterThan => |token| {},
                            ]);

                            return Some(
                                Type::Generic {
                                    typ: Ident {
                                        span: Span {
                                            input: token.span.input,
                                            range: token.span.range.clone(),
                                        },
                                    },
                                    generics,
                                }
                            );
                        }
                        _ => {
                            return Some(Type::Standard {
                                typ: Ident {
                                    span: token.span,
                                },
                            });
                        }
                    }
                },
            ]);

            None
        }
    }

    #[test]
    fn test_type_array() {
        let input = "[Int]";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Type::Array {
                typ: Box::new(Type::Standard {
                    typ: Ident {
                        span: Span { input, range: 1..4 },
                    },
                }),
            }),
            parser.parse::<Type>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_type_generic() {
        let input = "Edge <Int>";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Type::Generic {
                typ: Ident {
                    span: Span { input, range: 0..4 },
                },
                generics: vec![Type::Standard {
                    typ: Ident {
                        span: Span { input, range: 6..9 },
                    },
                }],
            }),
            parser.parse::<Type>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_type_standard() {
        let input = "Int";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Type::Standard {
                typ: Ident {
                    span: Span { input, range: 0..3 },
                },
            }),
            parser.parse::<Type>(),
            "{:?}",
            parser.errors,
        );
    }

    #[derive(Debug, PartialEq)]
    pub struct Ident<'i> {
        pub span: Span<'i>,
    }
}

pub mod lexer {
    use {
        super::Span,
        std::{iter::Peekable, str::CharIndices},
    };

    #[derive(Debug)]
    pub struct Token<'i> {
        pub span: Span<'i>,
        pub kind: TokenKind,
    }

    #[derive(Debug, PartialEq)]
    pub enum TokenKind {
        // implementation
        Ident,

        // keywords
        Enum,
        Service,
        Type,

        // symbols
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
}

// fn main() {
//     println!(
//         "{:#?}",
//         parser::Parser::new(lexer::Lexer::new(
//             r#"Connection <T> :: type {
//     edges: [Edge <T>]
//     page_info: PageInfo
// }"#
//         ))
//         .parse::<parser::File>()
//     );
// }
