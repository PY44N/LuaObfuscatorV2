use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLoadConst {
    instruction: Instruction,
}

impl OpLoadConst {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLoadConst {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = inst.const"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
