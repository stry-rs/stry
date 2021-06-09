use {
    std::io::{self, Write},
    swc_ecma_ast as js,
};

pub fn convert<W>(writer: &mut W, expr: &js::Expr) -> io::Result<()>
where
    W: Write,
{
    match expr {
        js::Expr::This(_) => {}
        js::Expr::Array(_) => {}
        js::Expr::Object(_) => {}
        js::Expr::Fn(_) => {}
        js::Expr::Unary(_) => {}
        js::Expr::Update(_) => {}
        js::Expr::Bin(_) => {}
        js::Expr::Assign(_) => {}
        js::Expr::Member(_) => {}
        js::Expr::Cond(_) => {}
        js::Expr::Call(_) => {}
        js::Expr::New(_) => {}
        js::Expr::Seq(_) => {}
        js::Expr::Ident(_) => {}
        js::Expr::Lit(lit) => {
            convert_expr_lit(writer, lit)?;
        }
        js::Expr::Tpl(_) => {}
        js::Expr::TaggedTpl(_) => {}
        js::Expr::Arrow(_) => {}
        js::Expr::Class(_) => {}
        js::Expr::Yield(_) => {}
        js::Expr::MetaProp(_) => {}
        js::Expr::Await(_) => {}
        js::Expr::Paren(_) => {}
        js::Expr::PrivateName(_) => {}
        js::Expr::OptChain(_) => {}
        js::Expr::Invalid(_) => {}
        // ignore all these
        // svelte outputs valid js and not jsx or ts
        js::Expr::JSXMember(_) => {}
        js::Expr::JSXNamespacedName(_) => {}
        js::Expr::JSXEmpty(_) => {}
        js::Expr::JSXElement(_) => {}
        js::Expr::JSXFragment(_) => {}
        js::Expr::TsTypeAssertion(_) => {}
        js::Expr::TsConstAssertion(_) => {}
        js::Expr::TsNonNull(_) => {}
        js::Expr::TsAs(_) => {}
    }

    Ok(())
}

pub fn convert_expr_lit<W>(writer: &mut W, lit: &js::Lit) -> io::Result<()>
where
    W: Write,
{
    match lit {
        js::Lit::Str(_) => {}
        js::Lit::Bool(js::Bool { value, .. }) => {
            write!(writer, "{}", value)?;
        }
        js::Lit::Null(_) => {}
        js::Lit::Num(_) => {}
        js::Lit::BigInt(_) => {}
        js::Lit::Regex(_) => {}
        // ignore all these
        // svelte outputs valid js and not jsx or ts
        js::Lit::JSXText(_) => {}
    }

    Ok(())
}
