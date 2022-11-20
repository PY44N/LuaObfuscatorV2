use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTailCall {
    instruction: Instruction,
}

impl OpTailCall {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTailCall {
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
