use std::{env, fs};

mod ast;
// mod check;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = fs::read_to_string(&args[1]).expect("Couldn't open file");
    let toks = token::tokenize(code);
    // println!("{:?}", toks);
    let ast = ast::make_ast(&toks);
    println!("{:?}", ast);
}
