use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpForPrep {
    instruction: Instruction,
}

impl OpForPrep {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpForPrep {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        -- local init, limit, step

        -- *: Possible additional error checking
        -- init = assert(tonumber(memory[A]), '`for` initial value must be a number')
        -- limit = assert(tonumber(memory[A + 1]), '`for` limit must be a number')
        -- step = assert(tonumber(memory[A + 2]), '`for` step must be a number')

        local init = Tonumber(memory[A])
        local limit = Tonumber(memory[A + 1])
        local step = Tonumber(memory[A + 2])

        memory[A] = init - step
        memory[A + 1] = limit
        memory[A + 2] = step

        pc = pc + inst.sBx"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
