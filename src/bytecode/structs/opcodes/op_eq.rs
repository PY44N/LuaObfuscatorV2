use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpEq {
    instruction: Instruction,
}

impl OpEq {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpEq {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
