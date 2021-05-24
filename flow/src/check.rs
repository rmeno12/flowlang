use crate::token::Token;

pub fn check(tokens: &[Token]) -> Result<(), &'static str> {
    // check devices
    let out = devices(&tokens)?;
    // check blocks
    let out = blocks(&tokens[out..], out)?;
    Ok(())
}

fn devices(tokens: &[Token]) -> Result<usize, &'static str> {
    let mut out = 0;
    while out < tokens.len() {
        match &tokens[out] {
            Token::Actuator | Token::Sensor => {
                let kind = &tokens[out];
                out += 1;
                if let Token::Identifier(name) = &tokens[out] {
                } else {
                    return Err("Expected identifier");
                }
                out += 1;
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
    }
    Ok(0)
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
