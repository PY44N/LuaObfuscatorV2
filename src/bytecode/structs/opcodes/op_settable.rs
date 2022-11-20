use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetTable {
    instruction: Instruction,
}

impl OpSetTable {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetTable {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
