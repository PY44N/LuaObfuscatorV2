use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpConcat {
    instruction: Instruction,
}

impl OpConcat {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpConcat {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
