use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpReturn {
    instruction: Instruction,
}

impl OpReturn {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpReturn {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local B = inst.B
        local len

        if B == 0 then
            len = top_index - A + 1
        else
            len = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return TableUnpack(memory, A, A + len - 1)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
