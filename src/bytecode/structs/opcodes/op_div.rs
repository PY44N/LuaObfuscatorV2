use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpDiv {
    instruction: Instruction,
}

impl OpDiv {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpDiv {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
