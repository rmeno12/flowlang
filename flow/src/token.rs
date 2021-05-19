#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Value(f64),
    Comparator(String),
    Actuator,
    Sensor,
    StartBlock,
    EndBlock,
    Set,
    Goto,
    Wait,
    If,
    Else,
    Any,
    All,
    Tab,
    Newline,
    Colon,
    ConditionStart,
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let words = split_code(code);
    for word in words {
        let to_add = match word.as_str() {
            "sensor" => Token::Sensor,
            "actuator" => Token::Actuator,
            "block" => Token::StartBlock,
            "endblock" => Token::EndBlock,
            "set" => Token::Set,
            "goto" => Token::Goto,
            "wait" => Token::Wait,
            "if" => Token::If,
            "else" => Token::Else,
            "any" => Token::Any,
            "all" => Token::All,
            "    " => Token::Tab,
            "\n" => Token::Newline,
            ":" => Token::Colon,
            "-" => Token::ConditionStart,
            "<" | ">" | "<=" | ">=" | "=" => Token::Comparator(word),
            _ => {
                let chars: Vec<char> = word.chars().collect();
                let first = chars[0];
                if first.is_digit(10) {
                    Token::Value(word.parse().unwrap())
                } else {
                    Token::Identifier(word)
                }
            }
        };
        tokens.push(to_add);
    }
    tokens
}

fn split_code(code: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut idx = 0;
    let chars: Vec<char> = code.chars().collect();
    while idx < code.chars().count() {
        // TODO: comparators
        match &chars[idx] {
            '\t' | '\n' | '-' | ':' | '=' => {
                out.push((&chars[idx]).to_string());
                idx += 1;
            }
            '<' | '>' => {
                if chars[idx + 1] == '=' {
                    out.push((&chars[idx..idx + 2]).iter().collect());
                    idx += 2;
                } else {
                    out.push((&chars[idx]).to_string());
                    idx += 1;
                }
            }
            '0'..='9' => {
                let mut new_idx = idx;
                while (&chars[new_idx]).is_digit(10) || chars[new_idx] == '.' {
                    new_idx += 1;
                }
                out.push((&chars[idx..new_idx]).iter().collect());
                idx = new_idx;
            }
            'A'..='z' => {
                let mut new_idx = idx;
                while (&chars[new_idx]).is_alphanumeric() || chars[new_idx] == '_' {
                    new_idx += 1;
                }
                out.push((&chars[idx..new_idx]).iter().collect());
                idx = new_idx;
            }
            ' ' => {
                let nextfour: String = (&chars[idx..idx + 4]).iter().collect();
                if nextfour.eq("    ") {
                    out.push(String::from("    "));
                    idx += 4;
                } else {
                    idx += 1;
                }
            }
            _ => {
                idx += 1;
            }
        }
    }
    out
}
