mod expr;
mod stmt;

use {swc_ecma_ast as js, std::io::{self, Write}};

pub fn convert_module<W>(writer: &mut W, module: &js::Module) -> io::Result<()>
where
    W: Write,
{
    for stmt in &module.body {
        match stmt {
            js::ModuleItem::ModuleDecl(_) => {}
            js::ModuleItem::Stmt(stmt) => stmt::convert(writer, stmt)?,
        }
    }

    Ok(())
}
