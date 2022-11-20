use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpClosure {
    instruction: Instruction,
}

impl OpClosure {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpClosure {
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
