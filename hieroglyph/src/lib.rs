pub mod generator;

pub mod lexer;
pub mod parser;

use std::ops::Range;

#[derive(PartialEq)]
pub struct Span<'i> {
    pub input: &'i str,
    pub range: Range<usize>,
}

impl<'i> Span<'i> {
    pub fn as_str(&self) -> &'i str {
        &self.input[self.range.clone()]
    }
}

impl<'i> std::fmt::Debug for Span<'i> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Span")
            .field("range", &self.range)
            .field("text", &&self.input[self.range.clone()])
            .finish()
    }
}

const TEXT: &str = r#"
Connection <T> :: type {
    edges: [Edge <T>]
    page_info: PageInfo
}
"#;

fn main() {
    let file = parser::Parser::new(lexer::Lexer::new(TEXT))
        .parse::<parser::File>()
        .unwrap();

    println!("{:#?}", file);

    generator::rust::rust(&mut std::io::stdout().lock(), &file).unwrap();
}
