use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use parser::Program;
use std::env;

mod parser;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

fn main() {
    let lexerdef = calc_l::lexerdef();
    // let args: Vec<String> = env::args().collect();
    let lexer = lexerdef.lexer("const msg = true  let ss = false, yy = null ");
    let (res, errs) = calc_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &calc_y::token_epp));
    }
    match res {
        Some(r) => {
            let program = Program::new(r.unwrap());
            let j = serde_json::to_string_pretty(&program).unwrap();
            println!("{}", j)
        }
        _ => eprintln!("Unable to evaluate expression."),
    }
}
