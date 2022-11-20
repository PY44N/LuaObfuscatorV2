use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpNot {
    instruction: Instruction,
}

impl OpNot {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpNot {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
