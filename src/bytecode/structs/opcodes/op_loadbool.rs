use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLoadBool {
    instruction: Instruction,
}

impl OpLoadBool {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLoadBool {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
