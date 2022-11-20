use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetUpval {
    instruction: Instruction,
}

impl OpSetUpval {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetUpval {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
