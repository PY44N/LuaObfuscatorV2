use crate::bytecode::{
    enums::{instruction_type::InstructionType, opcode_type::OpcodeType},
    structs::{instruction::Instruction, opcode::Opcode},
};

pub struct OpGetUpval {
    pub data: u32,
    pub opcode: OpcodeType,
    pub instruction_type: InstructionType,
    pub data_a: u8,
    pub data_b: i128,
    pub data_c: i64,
}

impl OpGetUpval {
    pub fn new(instruction: Instruction) -> Self {
        Self {
            data: instruction.data,
            opcode: instruction.opcode,
            instruction_type: instruction.instruction_type,
            data_a: instruction.data_a,
            data_b: instruction.data_b,
            data_c: instruction.data_c,
        }
    }
}

impl Opcode for OpGetUpval {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
