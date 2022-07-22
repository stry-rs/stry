use hieroglyph::{
    lexer::Lexer,
    parser::{Field, Function, Ident, Parser, Stmt, Type},
    Span,
};

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
        parser.errors(),
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
        parser.errors(),
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
        parser.errors(),
    );
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
        parser.errors(),
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
        parser.errors(),
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
        parser.errors(),
    );
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
        parser.errors(),
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
        parser.errors(),
    );
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
        parser.errors(),
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
        parser.errors(),
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
        parser.errors(),
    );
}
