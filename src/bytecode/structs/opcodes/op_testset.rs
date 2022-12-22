use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTestSet {
    instruction: Instruction,
}

impl OpTestSet {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTestSet {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local B = inst.B

        if (not memory[B]) ~= (inst.C ~= 0) then
            memory[A] = memory[B]
            pc = pc + code[pc].sBx
        end
        pc = pc + 1"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
