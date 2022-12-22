use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLoadNil {
    instruction: Instruction,
}

impl OpLoadNil {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLoadNil {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "for i = inst.A, inst.B do memory[i] = nil end"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
