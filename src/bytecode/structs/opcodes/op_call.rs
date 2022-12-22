use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpCall {
    instruction: Instruction,
}

impl OpCall {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpCall {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local B = inst.B
        local C = inst.C
        local params

        if B == 0 then
            params = top_index - A
        else
            params = B - 1
        end

        local ret_list = TablePack(memory[A](TableUnpack(memory, A + 1, A + params)))
        local ret_num = ret_list.n

        if C == 0 then
            top_index = A + ret_num - 1
        else
            ret_num = C - 1
        end

        TableMove(ret_list, 1, ret_num, A, memory)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
