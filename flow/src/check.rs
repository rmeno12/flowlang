use crate::token::Token;

pub fn check(tokens: &[Token]) -> Result<(), &'static str> {
    // check devices
    let mut out = devices(&tokens)?;

    // consume newline
    if let Token::Newline = &tokens[out] {
        out += 1;
    } else {
        return Err("Expected newline");
    }

    // check blocks
    out = blocks(&tokens[out..], out)?;
    if out < tokens.len() {
        Err("Unexpcted tokens")
    } else {
        Ok(())
    }
}

fn devices(tokens: &[Token]) -> Result<usize, &'static str> {
    let mut out = 0;
    while out < tokens.len() {
        match &tokens[out] {
            Token::Actuator | Token::Sensor => {
                out += 1;

                // consume name of device
                if let Token::Identifier(name) = &tokens[out] {
                    out += 1;
                } else {
                    return Err("Expected identifier");
                }

                // consume newline
                if let Token::Newline = &tokens[out] {
                    out += 1;
                } else {
                    return Err("Expected newline");
                }
            }
            _ => {
                break;
            }
        }
    }
    Ok(out)
}

fn blocks(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    let mut out = start;
    while out < tokens.len() {
        // consume the startblock
        if let Token::StartBlock = tokens[out] {
            out += 1;
        } else {
            return Err("Unexpected token");
        }

        out = block(tokens, out)?;

        // consume the endblock
        if let Token::EndBlock = &tokens[out] {
            out += 1;
        } else {
            return Err("Unexpected token");
        }

        // consume newline
        if let Token::Newline = &tokens[out] {
            out += 1;
        } else {
            return Err("Expected newline");
        }

        // consume extra newlines
        while let Token::Newline = &tokens[out] {
            out += 1;
        }
    }
    Ok(out)
}

fn block(tokens: &[Token], start: usize) -> Result<usize, &'static str> {
    let mut out = start;

    // consume the block name (maybe move this up to block()?)
    if let Token::Identifier(name) = &tokens[out] {
        out += 1;
    } else {
        return Err("Unexpected token");
    }

    // TODO: all this logic + respective functions
    while out < tokens.len() {
        match &tokens[out] {
            Token::EndBlock => {
                break;
            }
            Token::Set => {}
            Token::Goto => {}
            Token::Wait => {}
            Token::If => {}
            Token::Newline => {
                out += 1;
            }
            _ => {
                return Err("Unexpected token");
            }
        }
    }

    Ok(out)
}

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
        use crate::token;
        let code = std::fs::read_to_string("tests/simple.fl").expect("Couldn't open file");
        let toks = token::tokenize(code);
        assert!(check(&toks).is_ok());
    }
}
