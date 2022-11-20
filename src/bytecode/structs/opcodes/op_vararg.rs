use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpVarArg {
    instruction: Instruction,
}

impl OpVarArg {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpVarArg {
    fn get_obfuscated(&self) -> String {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
