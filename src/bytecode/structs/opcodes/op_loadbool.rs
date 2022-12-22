use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLoadBool {
    instruction: Instruction,
}

impl OpLoadBool {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLoadBool {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "memory[inst.A] = inst.B ~= 0

        if inst.C ~= 0 then pc = pc + 1 end"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
