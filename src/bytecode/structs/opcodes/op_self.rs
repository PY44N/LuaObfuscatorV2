use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSelf {
    instruction: Instruction,
}

impl OpSelf {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSelf {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A + 1] = memory[inst.B]
        memory[inst.A] = memory[inst.B][constantC(inst)]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
