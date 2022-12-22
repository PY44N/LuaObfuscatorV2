use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTest {
    instruction: Instruction,
}

impl OpTest {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTest {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "if (not memory[inst.A]) ~= (inst.C ~= 0) then pc = pc + code[pc].sBx end
        pc = pc + 1"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
