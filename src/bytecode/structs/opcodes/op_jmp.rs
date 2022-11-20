use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpJmp {
    instruction: Instruction,
}

impl OpJmp {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpJmp {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
