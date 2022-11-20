use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetList {
    instruction: Instruction,
}

impl OpSetList {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetList {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
