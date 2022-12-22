use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpForLoop {
    instruction: Instruction,
}

impl OpForLoop {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpForLoop {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local step = memory[A + 2]
        local index = memory[A] + step
        local limit = memory[A + 1]
        local loops

        if step == MathAbs(step) then
            loops = index <= limit
        else
            loops = index >= limit
        end

        if loops then
            memory[A] = index
            memory[A + 3] = index
            pc = pc + inst.sBx
        end"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
