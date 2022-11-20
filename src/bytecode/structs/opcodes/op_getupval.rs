use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpGetUpval {
    instruction: Instruction,
}

impl OpGetUpval {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpGetUpval {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
