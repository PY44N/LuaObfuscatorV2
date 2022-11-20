use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpCall {
    instruction: Instruction,
}

impl OpCall {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpCall {
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
