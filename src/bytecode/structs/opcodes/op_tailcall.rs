use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpTailCall {
    instruction: Instruction,
}

impl OpTailCall {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpTailCall {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local A = inst.A
        local B = inst.B
        local params

        if B == 0 then
            params = top_index - A
        else
            params = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return memory[A](TableUnpack(memory, A + 1, A + params))"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
