use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpNot {
    instruction: Instruction,
}

impl OpNot {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpNot {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = not memory[inst.B]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
