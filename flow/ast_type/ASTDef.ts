

type AST = {
    initBlockID: BlockID
    blocks: {
        [ID:string]:Instruction[]
    }
}



type Instruction = OP_Set | OP_If | OP_Goto | OP_Wait
    

    
type ActuatorID = string;
type ActuatorValue = any; // Assuming actuators can have multiple inputs
                          // and that input could be bool, string, or number
type SensorID = string;
type SensorValue = any;   // Assuming sensors can only have 1 output 
                          // and that input could be bool, string, or number

type Comparator = "=" | "<" | ">" | "<=" | ">="

type Condition = Condition_Base | ConditionSet_All | ConditionSet_Any
type Condition_Base = {
    conditionType: "BASE",
    sensor: SensorID,
    comparison: Comparator,
    value: SensorValue
}
type ConditionSet_Any = {
    conditionType: "SET_ANY",
    conditions: Condition[]
}
type ConditionSet_All = {
    conditionType: "SET_ALL",
    conditions: Condition[]
}

type BlockID = string

type OP_Set = {
    opcode: "SET", 
    arguments: {
        actuator: ActuatorID,
        value: ActuatorValue
    } 
}

type OP_Wait = {
    opcode: "WAIT",
    arguments: {
        condition: Condition
    }
}

type OP_If = {
    opcode: "IF",
    arguments: {
        condition: Condition,
        instructions_true_case: Instruction[],
        instructions_false_case: Instruction[]
    }
}

type OP_Goto = {
    opcode: "GOTO",
    arguments: {
        block: BlockID
    }
}

