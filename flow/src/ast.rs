use std::collections::HashMap;

use crate::token::Token;

#[derive(Debug)]
pub struct AST {
    first_block_name: String,
    devices: Vec<Device>,
    blocks: HashMap<String, Block>,
}

#[derive(Debug)]
pub enum Device {
    Actuator(Actuator),
    Sensor(Sensor),
}

// for now actuators/sensors can only be floats internally
#[derive(Debug)]
pub struct Actuator {
    name: String,
    min: f64,
    max: f64,
}

#[derive(Debug)]
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
    Base(Sensor),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}

pub fn make_ast(tokens: &[Token]) -> Result<AST, &'static str> {
    let (devices, mut idx) = make_devices(&tokens)?;
    println!("{:?}", devices);

    // consume newline
    if let Token::Newline = &tokens[idx] {
        idx += 1;
    } else {
        return Err("Expected newline after end of device list");
    }

    let (blocks, idx) = make_blocks(&tokens, idx)?;

    Ok(AST {
        first_block_name: String::from("hi"),
        devices,
        blocks,
    })
}

fn make_devices(tokens: &[Token]) -> Result<(Vec<Device>, usize), &'static str> {
    let mut outsize = 0;
    let mut devices: Vec<Device> = Vec::new();
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

                devices.push(match devkind {
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
                        name: dev_name,
                        min: 0.0,
                        max: 0.0,
                    }),
                });
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
) -> Result<(HashMap<String, Block>, usize), &'static str> {
    let mut idx = start;
    let mut blocks: HashMap<String, Block> = HashMap::new();
    while idx < tokens.len() {
        // consume the startblock
        if let Token::StartBlock = &tokens[idx] {
            idx += 1;
        } else {
            return Err("Expected block declaration");
        }

        let (block_name, block, newidx) = make_block(tokens, idx)?;
        idx = newidx;
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
    Ok((HashMap::new(), 0))
}

fn make_block(tokens: &[Token], start: usize) -> Result<(String, Block, usize), &'static str> {
    let mut idx = start;
    let mut ops: Vec<Operation> = Vec::new();
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

    // TODO: all this logic + respective functions
    while idx < tokens.len() {
        match &tokens[idx] {
            Token::EndBlock => {
                break;
            }
            Token::Set => {}
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

    Ok((block_name, Block { ops }, idx))
}
