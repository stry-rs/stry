use {
    html5ever::{driver::ParseOpts, parse_document, tendril::TendrilSink},
    std::io,
};

#[cfg(feature = "with-arcdom")]
use markup5ever_arcdom::{ArcDom as Dom, Handle, NodeData};
#[cfg(feature = "with-rcdom")]
use markup5ever_rcdom::{Handle, NodeData, RcDom as Dom};

pub fn convert(body: impl AsRef<str>) -> io::Result<String> {
    let mut body = body.as_ref().as_bytes();

    let dom = parse_document(Dom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut body)?;

    let mut buf = String::with_capacity(body.len() / 6);

    walk(&mut buf, &dom.document);

    Ok(buf)
}

fn walk(buf: &mut String, document: &Handle) {
    match &document.data {
        NodeData::Comment { .. }
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. } => {}
        NodeData::Document => {
            for node in document.children.borrow().iter() {
                walk(buf, node)
            }
        }
        NodeData::Text { contents } => {
            let mut prev = buf.is_empty() || buf.ends_with(' ') || buf.ends_with('\n');

            for c in contents.borrow().chars() {
                match c {
                    ' ' | '\n' => {
                        if !prev {
                            prev = true;

                            buf.push(' ');
                        }
                    }
                    _ => {
                        prev = false;

                        buf.push(c);
                    }
                }
            }
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.to_lowercase();
            let attrs = attrs.borrow();

            match tag.as_str() {
                "head" | "style" | "script" => {}
                _ => {
                    match tag.as_str() {
                        "h1" => buf.push_str("# "),
                        "h2" => buf.push_str("## "),
                        "h3" => buf.push_str("### "),
                        "h4" => buf.push_str("#### "),
                        "h5" => buf.push_str("##### "),
                        "h6" => buf.push_str("###### "),
                        "hr" => {
                            newline(buf);

                            buf.push_str("---");

                            newline(buf);
                        }
                        "br" => double_newline(buf),
                        "div" | "p" => {
                            double_newline(buf);
                        }
                        "img" => {
                            let mut src = "";
                            let mut alt = "no alt text";

                            for attr in attrs.iter() {
                                let name = attr.name.local.to_lowercase();

                                match name.as_str() {
                                    "alt" => {
                                        alt = &attr.value;
                                    }
                                    "src" => {
                                        src = &attr.value;
                                    }
                                    _ => {}
                                }
                            }

                            buf.push_str("![");
                            buf.push_str(alt);
                            buf.push_str("](");
                            buf.push_str(src);
                            buf.push(')');
                        }
                        // Non-empty elements
                        "a" => buf.push('['),
                        "b" | "strong" => buf.push_str("**"),
                        "i" | "em" => buf.push('*'),
                        _ => {}
                    }

                    for node in document.children.borrow().iter() {
                        walk(buf, node)
                    }

                    match tag.as_str() {
                        "a" => {
                            let mut url = "";

                            for attr in attrs.iter() {
                                let name = attr.name.local.to_lowercase();

                                if name.as_str() == "href" {
                                    url = &attr.value;
                                }
                            }

                            buf.push_str("](");
                            buf.push_str(url);
                            buf.push(')')
                        }
                        "b" | "strong" => buf.push_str("**"),
                        "i" | "em" => buf.push('*'),
                        _ => {}
                    }
                }
            }
        }
    }
}

#[inline]
fn trim_ending_whitespace(buf: &mut String) {
    while buf.ends_with(' ') || buf.ends_with('\t') {
        let end = buf.len() - 1;

        buf.remove(end);
    }
}

#[inline]
fn double_newline(buf: &mut String) {
    trim_ending_whitespace(buf);

    if !buf.ends_with("\n\n") {
        if buf.ends_with('\n') {
            buf.push('\n')
        } else if !buf.is_empty() {
            buf.push_str("\n\n")
        }
    }
}

#[inline]
fn newline(buf: &mut String) {
    trim_ending_whitespace(buf);

    if !buf.ends_with('\n') && !buf.is_empty() {
        buf.push('\n')
    }
}
