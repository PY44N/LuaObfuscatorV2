use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTest {
    instruction: Instruction,
}

impl OpTest {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTest {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
