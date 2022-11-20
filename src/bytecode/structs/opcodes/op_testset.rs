use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTestSet {
    instruction: Instruction,
}

impl OpTestSet {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTestSet {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
