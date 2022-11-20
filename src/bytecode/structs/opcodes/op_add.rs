use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpAdd {
    instruction: Instruction,
}

impl OpAdd {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpAdd {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
