use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpMod {
    instruction: Instruction,
}

impl OpMod {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpMod {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
