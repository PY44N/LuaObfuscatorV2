use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLe {
    instruction: Instruction,
}

impl OpLe {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLe {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
