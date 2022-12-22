use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLen {
    instruction: Instruction,
}

impl OpLen {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLen {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = #memory[inst.B]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
