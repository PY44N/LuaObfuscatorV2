use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpMove {
    instruction: Instruction,
}

impl OpMove {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpMove {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = memory[inst.B]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
