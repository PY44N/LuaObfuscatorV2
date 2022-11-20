use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTForLoop {
    instruction: Instruction,
}

impl OpTForLoop {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTForLoop {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
