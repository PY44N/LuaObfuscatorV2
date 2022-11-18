use crate::bytecode::enums::{
    instruction_type::{InstructionType, INSTRUCTION_TYPE_MAP},
    opcode_type::{OpcodeType, OPCODE_TYPE_MAP},
};

#[derive(Debug)]
pub struct Instruction {
    data: u32,
    opcode: OpcodeType,
    instruction_type: InstructionType,
}

impl Instruction {
    pub fn new(data: u32) -> Self {
        Self {
            data,
            opcode: OPCODE_TYPE_MAP[(data & 0x3f) as usize],
            instruction_type: INSTRUCTION_TYPE_MAP[(data & 0x3f) as usize],
        }
    }
}
