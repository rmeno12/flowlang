use std::collections::{HashMap, HashSet};

pub struct AST {
    first_block_name: String,
    devices: HashSet<Device>,
    blocks: HashMap<String, Block>,
}

pub enum Device {
    Actuator(Actuator),
    Sensor(Sensor),
}

// for now actuators/sensors can only be floats internally
pub struct Actuator {
    name: String,
    min: f64,
    max: f64,
}

pub struct Sensor {
    name: String,
    min: f64,
    max: f64,
}

pub struct Block {
    ops: Vec<Operation>,
}

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

pub enum Condition {
    Base(Sensor),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}
