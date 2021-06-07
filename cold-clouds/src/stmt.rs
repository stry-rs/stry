use {swc_ecma_ast as js, std::io::{self, Write}, crate::expr};

pub fn convert<W>(writer: &mut W, stmt: &js::Stmt) -> io::Result<()>
where
    W: Write,
{
    match stmt {
        js::Stmt::Block(block) => {
            for stmt in &block.stmts {
                convert(writer, stmt)?;
            }
        }
        js::Stmt::Empty(_) => {}
        js::Stmt::Debugger(_) => {}
        js::Stmt::With(_) => {}
        js::Stmt::Return(_) => {}
        js::Stmt::Labeled(_) => {}
        js::Stmt::Break(_) => {}
        js::Stmt::Continue(_) => {}
        js::Stmt::If(if_stmt) => convert_if(writer, if_stmt)?,
        js::Stmt::Switch(_) => {}
        js::Stmt::Throw(_) => {}
        js::Stmt::Try(_) => {}
        js::Stmt::While(_) => {}
        js::Stmt::DoWhile(_) => {}
        js::Stmt::For(_) => {}
        js::Stmt::ForIn(_) => {}
        js::Stmt::ForOf(_) => {}
        js::Stmt::Decl(_) => {}
        js::Stmt::Expr(_) => {}
    }

    Ok(())
}

pub fn convert_if<W>(writer: &mut W, if_stmt: &js::IfStmt) -> io::Result<()>
where
    W: Write,
{
    write!(writer, "if (")?;

    expr::convert(writer, &if_stmt.test)?;

    writeln!(writer, ") then")?;

    convert(writer, &if_stmt.cons)?;

    if let Some(stmt) = &if_stmt.alt {
        writeln!(writer, "else")?;

        convert(writer, stmt)?;
    }

    writeln!(writer, "end")?;

    Ok(())
}
