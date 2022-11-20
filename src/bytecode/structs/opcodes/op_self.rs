use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSelf {
    instruction: Instruction,
}

impl OpSelf {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSelf {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
