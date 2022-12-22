use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpGetGlobal {
    instruction: Instruction,
}

impl OpGetGlobal {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpGetGlobal {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = env[inst.const]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
