use crate::parser;

pub fn rust<W: std::io::Write>(writer: &mut W, file: &parser::File) -> std::io::Result<()> {
    fn write_type<W: std::io::Write>(writer: &mut W, typ: &parser::Type) -> std::io::Result<()> {
        match typ {
            parser::Type::Array { typ } => {
                write!(writer, "Vec<")?;
                write_type(writer, typ)?;
                write!(writer, ">")?;
            }
            parser::Type::Generic { typ, generics } => {
                write!(writer, "{}<", typ.span.as_str())?;
                let mut generics = generics.iter().peekable();
                while let Some(generic) = generics.next() {
                    write_type(writer, generic)?;
                    if generics.peek().is_some() {
                        write!(writer, ", ")?;
                    }
                }
                write!(writer, ">")?;
            }
            parser::Type::Standard { typ } => {
                write!(writer, "{}", typ.span.as_str())?;
            }
        }

        Ok(())
    }

    for stmt in &file.stmts {
        match stmt {
            parser::Stmt::Enum { name, variants } => {
                writeln!(writer, "pub enum {} {{", name.span.as_str())?;
                for variant in variants {
                    writeln!(writer, "    {},", variant.span.as_str())?;
                }
                writeln!(writer, "}}")?;
            }
            parser::Stmt::Service { name, functions } => todo!(),
            parser::Stmt::Type {
                name,
                generics,
                fields,
            } => {
                write!(writer, "pub struct {}", name.span.as_str())?;
                if !generics.is_empty() {
                    write!(writer, "<")?;
                    let mut generics = generics.iter().peekable();
                    while let Some(generic) = generics.next() {
                        write_type(writer, generic)?;
                        if generics.peek().is_some() {
                            write!(writer, ", ")?;
                        }
                    }
                    write!(writer, ">")?;
                }
                writeln!(writer, " {{")?;
                for field in fields {
                    write!(writer, "    pub {}: ", field.name.span.as_str())?;
                    write_type(writer, &field.typ)?;
                    writeln!(writer, ",")?;
                }
                writeln!(writer, "}}")?;
            }
        }
    }

    Ok(())
}
