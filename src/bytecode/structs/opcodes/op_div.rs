use crate::bytecode::{
    enums::instruction_type::InstructionType,
    structs::{instruction::Instruction, opcode::Opcode},
};

struct OpDiv {
    pub instruction_type: InstructionType,
    pub data_a: u8,
    pub data_b: i128,
    pub data_c: i64,
}

impl OpDiv {
    pub fn new(instruction: &Instruction) -> Self {
        Self {
            instruction_type: instruction.instruction_type,
            data_a: instruction.data_a,
            data_b: instruction.data_b,
            data_c: instruction.data_c,
        }
    }
}

impl Opcode for OpDiv {
    fn get_obfuscated(&self) -> String {
        todo!()
    }
}
