mod expr;
mod stmt;

use {
    std::io::{self, Write},
    swc_ecma_ast as js,
};

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
