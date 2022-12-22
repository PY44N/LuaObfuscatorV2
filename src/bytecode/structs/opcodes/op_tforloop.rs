use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTForLoop {
    instruction: Instruction,
}

impl OpTForLoop {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTForLoop {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local base = A + 3

        local vals = {memory[A](memory[A + 1], memory[A + 2])}

        TableMove(vals, 1, inst.C, base, memory)

        if memory[base] ~= nil then
            memory[A + 2] = memory[base]
            pc = pc + code[pc].sBx
        end

        pc = pc + 1"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
