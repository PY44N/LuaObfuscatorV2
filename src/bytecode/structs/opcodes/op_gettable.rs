use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpGetTable {
    instruction: Instruction,
}

impl OpGetTable {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpGetTable {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = memory[inst.B][constantC(inst)]"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
