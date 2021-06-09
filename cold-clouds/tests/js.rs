pub use cold_clouds::convert_module;

pub fn decode(source: &str) -> swc_ecma_ast::Module {
    use swc_common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        FileName, SourceMap,
    };
    use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Custom("test.js".into()), source.into());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.")
}

pub mod js {
    use super::*;

    #[test]
    fn test_simple() {
        let src = "if (true) {} else {}";
        let js = decode(src);
        let mut buf = Vec::new();
        convert_module(&mut buf, &js).unwrap();
        let lua = String::from_utf8(buf).unwrap();

        assert_eq!("", lua.as_str());
    }
}
