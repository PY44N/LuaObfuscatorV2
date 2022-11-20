use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpNewTable {
    instruction: Instruction,
}

impl OpNewTable {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpNewTable {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
