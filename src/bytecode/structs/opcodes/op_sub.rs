use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSub {
    instruction: Instruction,
}

impl OpSub {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSub {
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
