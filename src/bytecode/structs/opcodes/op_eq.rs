use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpEq {
    instruction: Instruction,
}

impl OpEq {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpEq {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "if (constantB(inst) == constantC(inst)) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

        pc = pc + 1"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
