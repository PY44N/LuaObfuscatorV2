use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpClose {
    instruction: Instruction,
}

impl OpClose {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpClose {
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
