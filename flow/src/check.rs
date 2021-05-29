use crate::token::Token;

fn set(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    Ok(0)
}

fn goto(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    Ok(0)
}

fn wait(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    Ok(0)
}

fn if_else(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        // use crate::token;
        // let code = std::fs::read_to_string("tests/simple.fl").expect("Couldn't open file");
        // let toks = token::tokenize(code);
        // assert!(check(&toks).is_ok());
    }
}
