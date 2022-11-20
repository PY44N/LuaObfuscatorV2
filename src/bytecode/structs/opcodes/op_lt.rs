use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpLt {
    instruction: Instruction,
}

impl OpLt {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpLt {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
