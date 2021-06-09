use std::collections::HashMap;

use crate::token::Token;

#[derive(Debug)]
pub struct AST {
    first_block_name: String,
    devices: HashMap<String, Device>,
    blocks: HashMap<String, Block>,
}

#[derive(Debug)]
pub enum Device {
    Actuator(Actuator),
    Sensor(Sensor),
}

// for now actuators/sensors can only be floats internally
#[derive(Debug, Clone)]
pub struct Actuator {
    name: String,
    min: f64,
    max: f64,
}

#[derive(Debug, Clone)]
pub struct Sensor {
    name: String,
    min: f64,
    max: f64,
}

#[derive(Debug)]
pub struct Block {
    ops: Vec<Operation>,
}

#[derive(Debug)]
pub enum Operation {
    Set {
        actuator: Actuator,
        value: f64,
    },
    Wait {
        condition: Condition,
    },
    IfElse {
        if_condition: Condition,
        if_actions: Vec<Operation>,
        else_actions: Option<Vec<Operation>>,
    },
    Goto {
        dest: String,
    },
}

#[derive(Debug)]
pub enum Condition {
    Base(Sensor, Comparator, f64),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}

#[derive(Debug)]
pub enum Comparator {
    LT,
    LTEQ,
    EQ,
    GT,
    GTEQ,
}

pub fn make_ast(tokens: &[Token]) -> Result<AST, &'static str> {
    let (devices, mut idx) = make_devices(&tokens)?;

    // consume newline
    if let Token::Newline = &tokens[idx] {
        idx += 1;
    } else {
        return Err("Expected newline after end of device list");
    }

    let (first_block_name, blocks, newidx) = make_blocks(&tokens, idx, &devices)?;
    idx = newidx;

    Ok(AST {
        first_block_name,
        devices,
        blocks,
    })
}

fn make_devices(tokens: &[Token]) -> Result<(HashMap<String, Device>, usize), &'static str> {
    let mut outsize = 0;
    let mut devices: HashMap<String, Device> = HashMap::new();
    while outsize < tokens.len() {
        match &tokens[outsize] {
            Token::Actuator | Token::Sensor => {
                let devkind = &tokens[outsize];
                outsize += 1;

                // consume name of device
                let dev_name: String;
                if let Token::Identifier(name) = &tokens[outsize] {
                    dev_name = name.clone();
                    outsize += 1;
                } else {
                    return Err("Expected identifier after device type");
                }
                // TODO: parse device ranges

                // consume newline
                if let Token::Newline = &tokens[outsize] {
                    outsize += 1;
                } else {
                    return Err("Expected newline after device declaration");
                }

                devices.insert(
                    dev_name.clone(),
                    match devkind {
                        Token::Actuator => Device::Actuator(Actuator {
                            name: dev_name,
                            min: f64::MIN,
                            max: f64::MAX,
                        }),
                        Token::Sensor => Device::Sensor(Sensor {
                            name: dev_name,
                            min: f64::MIN,
                            max: f64::MAX,
                        }),
                        _ => Device::Sensor(Sensor {
                            name: String::from(""),
                            min: 0.0,
                            max: 0.0,
                        }),
                    },
                );
            }
            _ => {
                break;
            }
        }
    }
    Ok((devices, outsize))
}

fn make_blocks(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
) -> Result<(String, HashMap<String, Block>, usize), &'static str> {
    let mut idx = start;
    let mut blocks: HashMap<String, Block> = HashMap::new();
    let mut first_block_name: Option<String> = None;
    while idx < tokens.len() {
        // consume the startblock
        if let Token::StartBlock = &tokens[idx] {
            idx += 1;
        } else {
            return Err("Expected block declaration");
        }

        let (block_name, block, newidx) = make_block(tokens, idx, &devices)?;
        idx = newidx;
        if let None = first_block_name {
            first_block_name = Some(block_name.clone());
        }
        blocks.insert(block_name, block);

        // consume the endblock
        if let Token::EndBlock = &tokens[idx] {
            idx += 1;
        } else {
            return Err("Expected \"endblock\" after block body");
        }

        // consume newline
        if let Token::Newline = &tokens[idx] {
            idx += 1;
        } else {
            return Err("Expected newline after \"endblock\"");
        }

        // consume extra newlines
        if idx < tokens.len() {
            while let Token::Newline = &tokens[idx] {
                idx += 1;
                if idx >= tokens.len() {
                    break;
                }
            }
        }
    }
    Ok((first_block_name.expect("No blocks provided"), blocks, idx))
}

fn make_block(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
) -> Result<(String, Block, usize), &'static str> {
    let mut idx = start;

    // consume the block name
    let block_name: String;
    if let Token::Identifier(name) = &tokens[idx] {
        block_name = name.clone();
        idx += 1;
    } else {
        return Err("Expected valid block name after \"block\" statement");
    }

    // consume newline
    if let Token::Newline = &tokens[idx] {
        idx += 1;
    } else {
        return Err("Expected newline after block name");
    }

    let (ops, newidx) = make_statements(tokens, idx, devices, 1, false)?;
    idx = newidx;

    Ok((block_name, Block { ops }, idx))
}

fn make_statements(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
    tabdepth: u8,
    ifelse: bool,
) -> Result<(Vec<Operation>, usize), &'static str> {
    let mut idx = start;
    let mut ops: Vec<Operation> = Vec::new();

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::EndBlock => {
                break;
            }
            Token::Newline => {
                idx += 1;
                continue;
            }
            _ => {}
        }

        if check_tabs(tokens, idx, tabdepth) {
            idx = consume_tabs(tokens, idx, tabdepth)?;
        } else {
            break;
        }

        println!("{:?}", &tokens[idx]);
        match &tokens[idx] {
            Token::Set => {
                idx += 1; // consume the set token
                let (set, newidx) = make_set(tokens, idx, &devices)?;
                ops.push(set);
                idx = newidx;

                // consume newline
                if let Token::Newline = &tokens[idx] {
                    idx += 1;
                } else {
                    return Err("Expected newline after set statement");
                }
            }
            Token::Goto => {
                idx += 1; // consume the goto token
                let (goto, newidx) = make_goto(tokens, idx)?;
                ops.push(goto);
                idx = newidx;

                // consume newline
                if let Token::Newline = &tokens[idx] {
                    idx += 1;
                } else {
                    return Err("Expected newline after set statement");
                }
            }
            Token::Wait => {
                idx += 1; // consume the wait

                // consume colon
                if let Token::Colon = &tokens[idx] {
                    idx += 1
                } else {
                    return Err("Expected colon after wait statement");
                }

                // consume newline
                if let Token::Newline = &tokens[idx] {
                    idx += 1;
                } else {
                    return Err("Expected newline after colon");
                }

                let (wait, newidx) = make_wait(tokens, idx, devices, tabdepth)?;
                ops.push(wait);
                idx = newidx;
            }
            Token::If => {
                idx += 1; // consume the if

                // consume colon
                if let Token::Colon = &tokens[idx] {
                    idx += 1
                } else {
                    return Err("Expected colon after if statement");
                }

                // consume newline
                if let Token::Newline = &tokens[idx] {
                    idx += 1;
                } else {
                    return Err("Expected newline after colon");
                }

                let (ifelse, newidx) = make_if(tokens, idx, devices, tabdepth)?;
                ops.push(ifelse);
                idx = newidx;
            }
            Token::Else => {
                println!("ifelse: {}", ifelse);
                if ifelse {
                    break;
                } else {
                    return Err("Unexpected token");
                }
            }
            _ => {
                println!("{:?}", &tokens[idx..]);
                return Err("Unexpected token");
            }
        }
    }

    Ok((ops, idx))
}

fn make_set(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
) -> Result<(Operation, usize), &'static str> {
    let mut idx = start;

    // consume device name
    let dev_name: String;
    if let Token::Identifier(name) = &tokens[idx] {
        dev_name = name.clone();
        idx += 1;
    } else {
        return Err("Expected valid device name after \"set\" statement");
    }

    if !devices.contains_key(&dev_name) {
        return Err("Expected valid device name after \"set\" statement");
    }

    let actuator: Actuator;
    if let Device::Actuator(act) = devices.get(&dev_name).unwrap() {
        actuator = act.clone();
    } else {
        return Err("Expected actuator device name after \"set\" statement");
    }

    // consume the value
    let dev_val: f64;
    if let Token::Value(val) = &tokens[idx] {
        dev_val = val.clone();
        idx += 1;
    } else {
        return Err("Expected valid device value after device name");
    }

    if !(dev_val <= actuator.max && dev_val >= actuator.min) {
        return Err("Expected value in range of device range");
    }

    Ok((
        Operation::Set {
            actuator,
            value: dev_val,
        },
        idx,
    ))
}

fn make_goto(tokens: &[Token], start: usize) -> Result<(Operation, usize), &'static str> {
    let mut idx = start;

    // TODO: check that the name is valid
    let block_name: String;
    if let Token::Identifier(name) = &tokens[idx] {
        block_name = name.clone();
        idx += 1;
    } else {
        return Err("Expected block name after \"goto\" statement");
    }

    Ok((Operation::Goto { dest: block_name }, idx))
}

fn make_wait(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
    tabdepth: u8,
) -> Result<(Operation, usize), &'static str> {
    let mut idx = start;

    let (condition, newidx) = make_condition(tokens, idx, devices, tabdepth + 1)?;
    idx = newidx;

    Ok((Operation::Wait { condition }, idx))
}

fn make_if(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
    tabdepth: u8,
) -> Result<(Operation, usize), &'static str> {
    let mut idx = start;

    let (if_condition, newidx) = make_condition(tokens, idx, devices, tabdepth + 1)?;
    idx = newidx;

    let (if_actions, newidx) = make_statements(tokens, idx, devices, tabdepth + 1, true)?;
    idx = newidx;

    // check if there is an else part
    let mut is_else = false;
    if idx + tabdepth as usize + 1 < tokens.len() {
        if let Token::Else = &tokens[idx + tabdepth as usize + 1] {
            is_else = true;
            idx += tabdepth as usize + 2;

            // consume colon
            if let Token::Colon = &tokens[idx] {
                idx += 1
            } else {
                return Err("Expected colon after else statement");
            }

            // consume newline
            if let Token::Newline = &tokens[idx] {
                idx += 1;
            } else {
                return Err("Expected newline after colon");
            }
        }
    }

    let mut else_actions: Option<Vec<Operation>> = None;

    if is_else {
        let (actions, newidx) = make_statements(tokens, idx, devices, tabdepth + 1, false)?;
        idx = newidx;
        else_actions = Some(actions);
    }

    Ok((
        Operation::IfElse {
            if_condition,
            if_actions,
            else_actions,
        },
        idx,
    ))
}

fn make_condition(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
    tabdepth: u8,
) -> Result<(Condition, usize), &'static str> {
    let mut idx = start;

    idx = consume_tabs(tokens, idx, tabdepth)?;

    // consume condition start
    // println!("{:?}", &tokens[idx..]);
    if let Token::ConditionStart = &tokens[idx] {
        idx += 1;
    } else {
        return Err("Expected \"-\" to mark beginning of condition");
    }

    match &tokens[idx] {
        Token::Any | Token::All => {
            let kind = &tokens[idx];
            idx += 1; // consume the any/all

            // consume colon
            if let Token::Colon = &tokens[idx] {
                idx += 1
            } else {
                return Err("Expected colon after wait statement");
            }

            // consume newline
            if let Token::Newline = &tokens[idx] {
                idx += 1;
            } else {
                return Err("Expected newline after colon");
            }

            let mut conditions: Vec<Condition> = Vec::new();
            while idx + tabdepth as usize + 1 < tokens.len() {
                if let Token::ConditionStart = &tokens[idx + tabdepth as usize + 1] {
                    let (condition, newidx) = make_condition(tokens, idx, devices, tabdepth + 1)?;
                    idx = newidx;
                    conditions.push(condition);
                } else {
                    break;
                }
            }

            if conditions.len() == 0 {
                Err("Expected conditions after any/all statement")
            } else {
                match kind {
                    Token::Any => Ok((Condition::Any(conditions), idx)),
                    Token::All => Ok((Condition::All(conditions), idx)),
                    _ => Err("Error in parsing, please report this bug"),
                }
            }
        }
        Token::Identifier(name) => {
            let dev_name = name.clone();
            idx += 1;

            if !devices.contains_key(&dev_name) {
                return Err("Expected valid device name after condition start");
            }

            let sensor: Sensor;
            if let Device::Sensor(sens) = devices.get(&dev_name).unwrap() {
                sensor = sens.clone();
            } else {
                return Err("Expected sensor device name after condition start");
            }

            let comparator: Comparator;
            if let Token::Comparator(comp) = &tokens[idx] {
                match comp.as_str() {
                    "<" => {
                        comparator = Comparator::LT;
                    }
                    "<=" => {
                        comparator = Comparator::LTEQ;
                    }
                    "=" => {
                        comparator = Comparator::EQ;
                    }
                    ">" => {
                        comparator = Comparator::GT;
                    }
                    ">=" => {
                        comparator = Comparator::GTEQ;
                    }
                    _ => {
                        return Err("Error in parsing, please report this bug");
                    }
                }
                idx += 1;
            } else {
                return Err("Expected comparator after device name in condition");
            }

            let val: f64;
            if let Token::Value(v) = &tokens[idx] {
                val = v.clone();
                idx += 1;
            } else {
                return Err("Expected valid value after comparator in condition");
            }

            // consume newline
            if let Token::Newline = &tokens[idx] {
                idx += 1;
            } else {
                return Err("Expected newline after colon");
            }

            Ok((Condition::Base(sensor, comparator, val), idx))
        }
        _ => Err("Expected device name, any, or all after condition start"),
    }
}

fn check_tabs(tokens: &[Token], start: usize, tabdepth: u8) -> bool {
    let mut tab_ok = true;

    if tokens.len() < start + tabdepth as usize {
        return false;
    }

    for i in 0..tabdepth {
        if let Token::Tab = &tokens[start + i as usize] {
        } else {
            tab_ok = false;
            break;
        }
    }

    tab_ok
}

fn consume_tabs(tokens: &[Token], start: usize, tabdepth: u8) -> Result<usize, &'static str> {
    let mut idx = start;

    println!(
        "tabdepth: {}, tokens: {:?}",
        tabdepth,
        &tokens[idx..=idx + tabdepth as usize]
    );
    for _ in 0..tabdepth {
        if let Token::Tab = &tokens[idx] {
            idx += 1;
        } else {
            return Err("Invalid indentation, expected a tab (4 spaces)");
        }
    }

    Ok(idx)
}
