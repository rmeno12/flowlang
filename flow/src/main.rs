use std::env;

mod token;

fn main() {
    // let args: Vec<String> = env::args().collect();
    use token::Token::*;
    let toks: Vec<token::Token> = vec![
        StartBlock,
        Identifier(String::from("blockname")),
        Set,
        Identifier(String::from("actuator")),
        Value(50.0),
        EndBlock,
    ];
    println!("{:?}", toks);
}
