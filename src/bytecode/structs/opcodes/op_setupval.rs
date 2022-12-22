use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetUpval {
    instruction: Instruction,
}

impl OpSetUpval {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetUpval {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local uv = upvals[inst.B]

        uv.store[uv.index] = memory[inst.A]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
