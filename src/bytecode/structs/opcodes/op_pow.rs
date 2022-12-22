use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpPow {
    instruction: Instruction,
}

impl OpPow {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpPow {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = constantB(inst) ^ constantC(inst)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
