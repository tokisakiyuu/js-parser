use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() -> Result<(), ()> {
    println!("cargo:rerun-if-changed=calc.l");
    println!("cargo:rerun-if-changed=calc.y");

    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("calc.y")
                .unwrap()
        })
        .lexer_in_src_dir("calc.l")
        .unwrap()
        .build()
        .unwrap();
    Ok(())
}
