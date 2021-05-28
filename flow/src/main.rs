use std::{env, fs, panic::panic_any};

mod ast;
mod check;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = fs::read_to_string(&args[1]).expect("Couldn't open file");
    let toks = token::tokenize(code);
    match check::check(&toks) {
        Ok(_val) => {
            println!("everything's flowy");
        }
        Err(e) => {
            panic_any(e);
        }
    }
}
