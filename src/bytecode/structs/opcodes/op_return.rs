use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpReturn {
    instruction: Instruction,
}

impl OpReturn {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpReturn {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
