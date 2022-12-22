use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpConcat {
    instruction: Instruction,
}

impl OpConcat {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpConcat {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local B = inst.B
        local str = memory[B]

        for i = B + 1, inst.C do str = str .. memory[i] end

        memory[inst.A] = str"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
