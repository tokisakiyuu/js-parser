use core::panic;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::{env, fs, path::Path};

mod parser;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args.last().expect("Please enter a file");

    let mut path = env::current_dir().unwrap();
    path.push(Path::new(input_path));
    let metadata = fs::metadata(&path).expect("File is not exists");
    if !metadata.is_file() {
        panic!("It is not a file")
    }

    let source = fs::read(path).expect("Cannot read input file");
    let input = match String::from_utf8(source) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let lexerdef = calc_l::lexerdef();
    let lexer = lexerdef.lexer(&input);
    let (result, errs) = calc_y::parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &calc_y::token_epp));
    }

    match result.expect("Unable to parse expression") {
        Ok(r) => {
            let j = serde_json::to_string_pretty(&r).unwrap();
            println!("{}", j)
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}
