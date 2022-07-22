use std::iter::{Filter, Peekable};

use super::{
    lexer::{Lexer, Token, TokenKind},
    Span,
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
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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

    pub fn errors(&self) -> &[Error] {
        self.errors.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct File<'i> {
    pub stmts: Vec<Stmt<'i>>,
}

impl<'i> Ast<'i> for File<'i> {
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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

                    // match lexer.peek()? {
                    //     token if token.kind == TokenKind::Comma => {
                    //         take!(lexer, errors: [
                    //             TokenKind::Comma => |_token| {},
                    //         ]);
                    //     }
                    //     _ => break,
                    // }

                    match lexer.peek()? {
                        token if token.kind == TokenKind::CloseBrace => break,
                        _ => {}
                    }
                }

                Some(Stmt::Type {
                    name,
                    generics: generics.unwrap_or_default(),
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

#[derive(Debug, PartialEq)]
pub struct Field<'i> {
    pub name: Ident<'i>,
    pub typ: Type<'i>,
}

impl<'i> Ast<'i> for Field<'i> {
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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

#[derive(Debug, PartialEq)]
pub struct Function<'i> {
    pub name: Ident<'i>,
    pub generics: Vec<Type<'i>>,
    pub fields: Vec<Field<'i>>,
    pub ret: Type<'i>,
}

impl<'i> Ast<'i> for Function<'i> {
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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
            generics: generics.unwrap_or_default(),
            fields,
            ret: typ,
        })
    }
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
    fn parse(lexer: &mut Peekable<FilteredLexer<'i>>, errors: &mut Vec<Error<'i>>) -> Option<Self>
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
                                generics: generics.unwrap_or_default(),
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

#[derive(Debug, PartialEq)]
pub struct Ident<'i> {
    pub span: Span<'i>,
}
