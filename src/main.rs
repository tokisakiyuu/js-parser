use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
pub mod lexer;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parse() {
    use grammar::ProgramParser;
    use lexer::Lexer;

    let lexer = Lexer::new("const message = \"Hello World\", num = 2333;");
    assert!(ProgramParser::new().parse(lexer).is_ok());
}
