use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

mod parser;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

// pub fn parse<'a>(
//     source: impl Into<&'a str>,
// ) -> (
//     Option<Result<Vec<parser::ASTNode>, ()>>,
//     Vec<lrpar::LexParseError<u32, lrlex::DefaultLexerTypes>>,
// ) {
//     let lexerdef = calc_l::lexerdef();
//     let lexer = lexerdef.lexer(source.into());
//     calc_y::parse(&lexer)
// }

// #[test]
// fn test() {
//     let source1 = "let aa = 123";
//     let (res, errs) = parse(source1);
//     for e in errs {
//         println!("{}", e.);
//     }
//     match res {
//         Some(r) => {
//             let program = Program::new(r.unwrap());
//             let j = serde_json::to_string_pretty(&program).unwrap();
//             println!("{}", j)
//         }
//         _ => eprintln!("Unable to evaluate expression."),
//     }
// }
