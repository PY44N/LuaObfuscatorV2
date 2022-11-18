use crate::bytecode::enums::opcode_type::{OpcodeType, OpcodeTypeEnum};

pub struct Instruction {
    data: u32,
    opcode: OpcodeTypeEnum,
}

impl Instruction {
    pub fn new(data: u32) -> Self {
        Self {
            data,
            opcode: OpcodeType[(data & 0x3f) as usize],
        }
    }
}
