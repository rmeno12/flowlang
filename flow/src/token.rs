#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Value(f64),
    Comparator(String),
    StartBlock,
    EndBlock,
    Set,
    Goto,
    Wait,
    If,
    Else,
    Any,
    All,
}
