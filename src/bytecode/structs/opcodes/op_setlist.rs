use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpSetList {
    instruction: Instruction,
}

impl OpSetList {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpSetList {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local C = inst.C
        local len = inst.B
        local tab = memory[A]
        local offset

        if len == 0 then len = top_index - A end

        if C == 0 then
            C = inst[pc].value
            pc = pc + 1
        end

        offset = (C - 1) * 50 --FIELDS_PER_FLUSH

        TableMove(memory, A + 1, A + len, offset + 1, tab)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
