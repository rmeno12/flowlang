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
        else_condition: Condition,
        else_actions: Vec<Operation>,
    },
    Goto {
        dest: String,
    },
}

#[derive(Debug)]
pub enum Condition {
    Base(Sensor, f64),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}

pub fn make_ast(tokens: &[Token]) -> Result<AST, &'static str> {
    let (devices, mut idx) = make_devices(&tokens)?;
    // println!("{:?}", devices);

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

    let (ops, newidx) = make_statements(tokens, idx, devices, 1)?;
    idx = newidx;

    Ok((block_name, Block { ops }, idx))
}

fn make_statements(
    tokens: &[Token],
    start: usize,
    devices: &HashMap<String, Device>,
    tabdepth: u8,
) -> Result<(Vec<Operation>, usize), &'static str> {
    let mut idx = start;
    let mut ops: Vec<Operation> = Vec::new();

    // TODO: all this logic + respective functions
    while idx < tokens.len() {
        // logic for consuming and checking tabs at beginning of line
        let mut tab_ok = true;
        let mut tabs = 0;
        for i in 0..tabdepth {
            if !tab_ok || idx >= tokens.len() {
                break;
            }
            if let Token::Tab = tokens[idx + i as usize] {
                tabs += 1;
            } else {
                tab_ok = false;
            }
        }
        idx += tabs;
        if !tab_ok {
            return Ok((ops, idx));
        }

        match &tokens[idx] {
            Token::EndBlock => {
                break;
            }
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
            Token::Goto => {}
            Token::Wait => {}
            Token::If => {}
            Token::Newline => {
                idx += 1;
            }
            _ => {
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
