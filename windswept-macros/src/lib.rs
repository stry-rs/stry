use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprForLoop, ExprIf, ExprMatch, Stmt};
use syn_rsx::{parse, Node, NodeType};

fn walk(size: &mut usize, nodes: Vec<Node>) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for node in nodes {
        match node.node_type {
            NodeType::Element => {
                let name = node.name_as_string().unwrap();
                *size += 1 + name.len() + 1;
                let attrs = walk(size, node.attributes).into_iter();
                tokens.extend(quote! {
                    write!(f, concat!("<", #name))?;
                    #( #attrs )*
                    write!(f, ">")?;
                });

                // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
                match name.as_str() {
                    "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                    | "meta" | "param" | "source" | "track" | "wbr" => continue,
                    _ => (),
                }

                let children = walk(size, node.children).into_iter();
                *size += 2 + name.len() + 1;
                tokens.extend(quote! {
                    #( #children )*
                    write!(f, concat!("</", #name, ">"))?;
                });
            }
            NodeType::Attribute => {
                let key = node.name_as_string().unwrap();
                *size += 1 + key.len();
                if let Some(value) = node.value {
                    *size += 1 + 2;
                    tokens.extend(quote! { write!(f, concat!(" ", #key, "=\"{}\""), #value)?; });
                } else {
                    tokens.extend(quote! { write!(f, concat!(" ", #key))?; });
                }
            }
            NodeType::Text => {
                let value = node.value_as_string().unwrap();
                *size += value.len();
                tokens.extend(quote! { write!(f, #value)?; });
            }
            NodeType::Comment => {
                let value = node.value_as_string().unwrap();
                *size += 5 + value.len() + 4;
                tokens.extend(quote! { write!(f, concat!("<!-- ", #value, " -->"))?; });
            }
            NodeType::Doctype => {
                let value = node.value_as_string().unwrap();
                *size += 10 + value.len() + 1;
                tokens.extend(quote! { write!(f, "<!DOCTYPE {}>", #value)?; });
            }
            NodeType::Fragment => {
                tokens.extend(walk(size, node.children).into_iter());
            }
            NodeType::Block => {
                let value = node.value.unwrap();

                match value {
                    Expr::Block(block_expr) if block_expr.block.stmts.len() == 1 => {
                        handle_special_exprs(&mut tokens, block_expr);
                    }
                    value => {
                        tokens.extend(quote! { ::windswept::Render::render_into({ #value }, f)?; });
                    }
                }
            }
        }
    }

    tokens
}

fn handle_special_exprs(tokens: &mut proc_macro2::TokenStream, block_expr: syn::ExprBlock) {
    match &block_expr.block.stmts[0] {
        Stmt::Expr(Expr::ForLoop(ExprForLoop {
            attrs,
            label,
            for_token,
            pat,
            in_token,
            expr,
            body,
        })) => {
            let attrs_iter = attrs.iter();

            tokens.extend(quote! {
                #( #attrs_iter )*
                #label #for_token #pat #in_token #expr {
                    ::windswept::Render::render_into({ #body }, f)?;
                }
            });
        }
        Stmt::Expr(Expr::If(ExprIf {
            attrs,
            if_token,
            cond,
            then_branch,
            else_branch,
        })) => {
            let else_branch = else_branch.as_ref().map(|(else_token, else_expr)| {
                quote! {
                    #else_token {
                        ::windswept::Render::render_into({ #else_expr }, f)?;
                    }
                }
            });

            let attrs_iter = attrs.iter();
            tokens.extend(quote! {
                #( #attrs_iter )*
                #if_token #cond {
                    ::windswept::Render::render_into({ #then_branch }, f)?;
                } #else_branch
            });
        }
        Stmt::Expr(Expr::Match(ExprMatch {
            attrs,
            match_token,
            expr,
            arms,
            ..
        })) => {
            let arms_iter = arms.iter().map(
                |syn::Arm {
                     attrs,
                     pat,
                     guard,
                     fat_arrow_token,
                     body,
                     comma,
                 }| {
                    let guard = guard.as_ref().map(|(token, expr)| quote! { #token #expr });
                    let attrs_iter = attrs.iter();
                    quote! {
                        #( #attrs_iter )*
                        #pat #guard #fat_arrow_token {
                            ::windswept::Render::render_into({ #body }, f)?;
                        } #comma
                    }
                },
            );

            let attrs_iter = attrs.iter();
            tokens.extend(quote! {
                #( #attrs_iter )*
                #match_token #expr {
                    #( #arms_iter )*
                }
            });
        }
        stmt => {
            tokens.extend(quote! { ::windswept::Render::render_into({ #stmt }, f)?; });
        }
    }
}

#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            let mut size = 0;
            let tokens = walk(&mut size, nodes);

            let body = quote! {
                (
                    #[allow(unused_braces)]
                    move |f: &mut dyn ::std::fmt::Write| -> ::std::result::Result<(), ::std::fmt::Error> {
                        use ::std::fmt::Write as _;
                        #tokens
                        Ok(())
                    },
                    #size,
                )
            };

            // panic!("{}", body.to_string());

            body
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}
