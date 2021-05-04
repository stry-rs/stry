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

        fn parse_generics(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Vec<Type<'i>>> {
            match lexer.peek()? {
                _token if _token.kind == TokenKind::LessThan => {
                    let mut generics: Vec<Type<'i>> = Vec::new();

                    take!(lexer, errors: [
                        TokenKind::LessThan => |token| {},
                    ]);

                    loop {
                        match lexer.peek()? {
                            token if token.kind == TokenKind::GreaterThan => break,
                            _ => {}
                        }

                        let typ: Type<'i> = Type::parse(lexer, errors)?;

                        generics.push(typ);

                        match lexer.peek()? {
                            token if token.kind == TokenKind::Comma => {
                                take!(lexer, errors: [
                                    TokenKind::Comma => |_token| {},
                                ]);
                            }
                            _ => break,
                        }
                    }

                    take!(lexer, errors: [
                        TokenKind::GreaterThan => |token| {},
                    ]);

                    Some(generics)
                }
                _token if _token.kind == TokenKind::Colon => None,
                _token => {
                    errors.push(Error {
                        span: Span {
                            input: _token.span.input,
                            range: _token.span.range.clone(),
                        },
                        got: _token.kind,
                        expected: vec![TokenKind::LessThan, TokenKind::Colon],
                    });

                    None
                }
            }
        }

        // parses: ident < generics > :: kind
        fn parse_header(
            function: bool,
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<(Ident<'i>, Option<Vec<Type<'i>>>, Token<'i>)> {
            take!(lexer, errors: [
                TokenKind::Ident => |ident| {
                    let name = Ident {
                        span: ident.span,
                    };

                    let generics: Option<Vec<Type<'i>>> = <Self as Ast>::parse_generics(lexer, errors);

                    take!(lexer, errors: [
                        TokenKind::Colon => |_token| {},
                    ]);
                    take!(lexer, errors: [
                        TokenKind::Colon => |_token| {},
                    ]);

                    let mut kind = None;

                    if function {
                        take!(lexer, errors: [
                            TokenKind::Function => |_kind| {
                                kind = Some(_kind);
                            },
                        ]);
                    } else {
                        take!(lexer, errors: [
                            TokenKind::Enum => |_kind| {
                                kind = Some(_kind);
                            },
                            TokenKind::Service => |_kind| {
                                kind = Some(_kind);
                            },
                            TokenKind::Type => |_kind| {
                                kind = Some(_kind);
                            },
                        ]);
                    }

                    return kind.map(|kind| (
                        name,
                        generics,
                        kind,
                    ));
                },
            ]);

            None
        }
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

    #[derive(Debug, PartialEq)]
    pub struct File<'i> {
        pub stmts: Vec<Stmt<'i>>,
    }

    impl<'i> Ast<'i> for File<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized,
        {
            let mut stmts = Vec::new();

            loop {
                match lexer.peek() {
                    Some(token) if token.kind == TokenKind::Ident => {}
                    _ => break,
                }

                let stmt = Stmt::parse(lexer, errors)?;

                stmts.push(stmt);

                match lexer.peek() {
                    Some(token) if token.kind == TokenKind::Ident => {}
                    _ => break,
                }
            }

            Some(File { stmts })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Stmt<'i> {
        Enum {
            name: Ident<'i>,
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

    impl<'i> Ast<'i> for Stmt<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized,
        {
            let (name, generics, kind) = <Self as Ast>::parse_header(false, lexer, errors)?;

            take!(lexer, errors: [
                TokenKind::OpenBrace => |_token| {},
            ]);

            let ret = match kind.kind {
                TokenKind::Enum => {
                    let mut variants = Vec::new();

                    loop {
                        match lexer.peek()? {
                            token if token.kind == TokenKind::CloseBrace => break,
                            _ => {}
                        }

                        take!(lexer, errors: [
                            TokenKind::Ident => |_token| {
                                variants.push(Ident {
                                    span: Span {
                                        input: _token.span.input,
                                        range: _token.span.range.clone(),
                                    },
                                });
                            },
                        ]);

                        match lexer.peek()? {
                            token if token.kind == TokenKind::Ident => {}
                            _ => break,
                        }
                    }

                    Some(Stmt::Enum { name, variants })
                }
                TokenKind::Service => {
                    let mut functions = Vec::new();

                    loop {
                        match lexer.peek()? {
                            token if token.kind == TokenKind::CloseBrace => break,
                            _ => {}
                        }

                        let function: Function<'i> = Function::parse(lexer, errors)?;

                        functions.push(function);

                        match lexer.peek()? {
                            token if token.kind == TokenKind::Comma => {
                                take!(lexer, errors: [
                                    TokenKind::Comma => |_token| {},
                                ]);
                            }
                            token if token.kind == TokenKind::Ident => {}
                            _ => break,
                        }
                    }

                    Some(Stmt::Service { name, functions })
                }
                TokenKind::Type => {
                    let mut fields = Vec::new();

                    loop {
                        match lexer.peek()? {
                            token if token.kind == TokenKind::CloseBrace => break,
                            _ => {}
                        }

                        let field: Field<'i> = Field::parse(lexer, errors)?;

                        fields.push(field);

                        match lexer.peek()? {
                            token if token.kind == TokenKind::Comma => {
                                take!(lexer, errors: [
                                    TokenKind::Comma => |_token| {},
                                ]);
                            }
                            _ => break,
                        }
                    }

                    Some(Stmt::Type {
                        name,
                        generics: generics.unwrap_or_else(Vec::new),
                        fields,
                    })
                }
                _ => None,
            };

            take!(lexer, errors: [
                TokenKind::CloseBrace => |_token| {},
            ]);

            ret
        }
    }

    #[test]
    fn test_stmt_enum() {
        let input = "Rating :: enum { Teen General }";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Stmt::Enum {
                name: Ident {
                    span: Span { input, range: 0..6 },
                },
                variants: vec![
                    Ident {
                        span: Span {
                            input,
                            range: 17..21,
                        },
                    },
                    Ident {
                        span: Span {
                            input,
                            range: 22..29,
                        },
                    },
                ],
            }),
            parser.parse::<Stmt>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_stmt_service() {
        let input = "Query :: service { age <P> :: fn (person: P) -> Int }";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Stmt::Service {
                name: Ident {
                    span: Span { input, range: 0..5 },
                },
                functions: vec![Function {
                    name: Ident {
                        span: Span {
                            input,
                            range: 19..22,
                        },
                    },
                    generics: vec![Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 24..25,
                            },
                        },
                    },],
                    fields: vec![Field {
                        name: Ident {
                            span: Span {
                                input,
                                range: 34..40,
                            },
                        },
                        typ: Type::Standard {
                            typ: Ident {
                                span: Span {
                                    input,
                                    range: 42..43,
                                },
                            },
                        },
                    },],
                    ret: Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 48..51,
                            },
                        },
                    },
                },],
            }),
            parser.parse::<Stmt>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_stmt_type() {
        let input = "Character <A> :: type { age: A }";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Stmt::Type {
                name: Ident {
                    span: Span { input, range: 0..9 },
                },
                generics: vec![Type::Standard {
                    typ: Ident {
                        span: Span {
                            input,
                            range: 11..12,
                        },
                    },
                },],
                fields: vec![Field {
                    name: Ident {
                        span: Span {
                            input,
                            range: 24..27,
                        },
                    },
                    typ: Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 29..30,
                            },
                        },
                    },
                },],
            }),
            parser.parse::<Stmt>(),
            "{:?}",
            parser.errors,
        );
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

    #[derive(Debug, PartialEq)]
    pub struct Function<'i> {
        pub name: Ident<'i>,
        pub generics: Vec<Type<'i>>,
        pub fields: Vec<Field<'i>>,
        pub ret: Type<'i>,
    }

    impl<'i> Ast<'i> for Function<'i> {
        fn parse(
            lexer: &mut Peekable<FilteredLexer<'i>>,
            errors: &mut Vec<Error<'i>>,
        ) -> Option<Self>
        where
            Self: Sized,
        {
            let (name, generics, _kind) = <Self as Ast>::parse_header(true, lexer, errors)?;

            take!(lexer, errors: [
                TokenKind::OpenParen => |_token| {},
            ]);

            let mut fields = Vec::new();

            loop {
                match lexer.peek()? {
                    token if token.kind == TokenKind::CloseParen => break,
                    _ => {}
                }

                let field: Field<'i> = Field::parse(lexer, errors)?;

                fields.push(field);

                match lexer.peek()? {
                    token if token.kind == TokenKind::Comma => {
                        take!(lexer, errors: [
                            TokenKind::Comma => |_token| {},
                        ]);
                    }
                    _ => break,
                }
            }

            take!(lexer, errors: [
                TokenKind::CloseParen => |_token| {},
            ]);

            take!(lexer, errors: [
                TokenKind::Minus => |_token| {},
            ]);

            take!(lexer, errors: [
                TokenKind::GreaterThan => |token| {},
            ]);

            let typ: Type<'i> = Type::parse(lexer, errors)?;

            Some(Function {
                name,
                generics: generics.unwrap_or_else(Vec::new),
                fields,
                ret: typ,
            })
        }
    }

    #[test]
    fn test_function_with() {
        let input = "age <P> :: fn (person: P) -> Int";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Function {
                name: Ident {
                    span: Span { input, range: 0..3 },
                },
                generics: vec![Type::Standard {
                    typ: Ident {
                        span: Span { input, range: 5..6 },
                    },
                },],
                fields: vec![Field {
                    name: Ident {
                        span: Span {
                            input,
                            range: 15..21,
                        },
                    },
                    typ: Type::Standard {
                        typ: Ident {
                            span: Span {
                                input,
                                range: 23..24,
                            },
                        },
                    }
                }],
                ret: Type::Standard {
                    typ: Ident {
                        span: Span {
                            input,
                            range: 29..32,
                        },
                    },
                },
            }),
            parser.parse::<Function>(),
            "{:?}",
            parser.errors,
        );
    }

    #[test]
    fn test_function_without() {
        let input = "age <P> :: fn () -> Int";
        let mut parser = Parser::new(Lexer::new(input));

        assert_eq!(
            Some(Function {
                name: Ident {
                    span: Span { input, range: 0..3 },
                },
                generics: vec![Type::Standard {
                    typ: Ident {
                        span: Span { input, range: 5..6 },
                    },
                },],
                fields: vec![],
                ret: Type::Standard {
                    typ: Ident {
                        span: Span {
                            input,
                            range: 20..23,
                        },
                    },
                },
            }),
            parser.parse::<Function>(),
            "{:?}",
            parser.errors,
        );
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
                            let generics = <Self as Ast>::parse_generics(lexer, errors);

                            return Some(
                                Type::Generic {
                                    typ: Ident {
                                        span: Span {
                                            input: token.span.input,
                                            range: token.span.range.clone(),
                                        },
                                    },
                                    generics: generics.unwrap_or_else(Vec::new),
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
