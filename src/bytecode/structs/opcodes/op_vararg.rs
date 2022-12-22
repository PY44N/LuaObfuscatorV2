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
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local len = inst.B

        if len == 0 then
            len = vararg.len
            top_index = A + len - 1
        end

        TableMove(vararg.list, 1, len, A, memory)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
