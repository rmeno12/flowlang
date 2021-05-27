pub struct AST {
    init_block_id: String,
    blocks: std::collections::HashMap<String, Block>,
}

pub type Block = Vec<Instruction>;

pub enum Instruction {
    SetOperation(SetOperation),
    WaitOperation(WaitOperation),
    IfOperation(IfOperation),
    GotoOperation(GotoOperation),
}

pub type ActuatorID = String;
pub type ActuatorValue = Vec<AnyValue>; // Assuming actuators can have multiple inputs

pub type SensorID = String;
pub type SensorValue = AnyValue; // Assuming sensors can only have 1 output

pub enum Condition {
    ConditionBase(ConditionBase),
    ConditionSet(ConditionSet),
}
pub enum Comparitor {
    EqualTo,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
pub struct ConditionBase {
    sensor: SensorID,
    comparison: Comparitor,
    value: SensorValue,
}
pub enum ConditionSetType {
    Any,
    All,
}
pub struct ConditionSet {
    set_type: ConditionSetType,
    conditions: Vec<Condition>,
}

pub struct SetOperation {
    actuator: ActuatorID,
    value: ActuatorValue,
}
pub struct WaitOperation {
    condition: Condition,
}
pub struct IfOperation {
    condition: Condition,
    exec_if_true: Block,
    exec_if_false: Block,
}
pub struct GotoOperation {
    block_id: String,
}

pub enum AnyValue {
    String(String),
    Boolean(bool),
    Number(f64),
}
