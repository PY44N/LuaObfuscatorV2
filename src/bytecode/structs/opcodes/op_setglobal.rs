use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetGlobal {
    instruction: Instruction,
}

impl OpSetGlobal {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetGlobal {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "env[inst.const] = memory[inst.A]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
