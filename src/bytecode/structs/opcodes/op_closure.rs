use crate::bytecode::structs::{instruction::Instruction, opcode::Opcode};

pub struct OpClosure {
    instruction: Instruction,
}

impl OpClosure {
    pub fn new(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl Opcode for OpClosure {
    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn get_obfuscated(&self) -> &str {
        "local sub = subs[inst.Bx + 1] -- offset for 1 based index
        local nups = sub.num_upval
        local uvlist

        if nups ~= 0 then
            uvlist = {}

            for i = 1, nups do
                local pseudo = code[pc + i - 1]

                if pseudo.op == 0 then -- @MOVE
                    uvlist[i - 1] = open_lua_upvalue(open_list, pseudo.B, memory)
                elseif pseudo.op == 4 then -- @GETUPVAL
                    uvlist[i - 1] = upvals[pseudo.B]
                end
            end

            pc = pc + nups
        end

        memory[inst.A] = lua_wrap_state(sub, env, uvlist)"
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}
