use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpPow {
    instruction: Instruction,
}

impl OpPow {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpPow {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
