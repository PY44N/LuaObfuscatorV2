use lua_deserializer::enums::opcode_type::OpcodeType;

pub fn get_opcode_string(opcode: &OpcodeType, opcode_list: &Vec<OpcodeType>) -> String {
    match opcode {
        OpcodeType::OpMove => "memory[inst[$A_REGISTER$]] = memory[inst[$B_REGISTER$]]".to_string(),
        OpcodeType::OpLoadConst => "memory[inst[$A_REGISTER$]] = inst[$CONSTANT$]".to_string(),
        OpcodeType::OpLoadBool => {
            "memory[inst[$A_REGISTER$]] = inst[$B_REGISTER$] ~= 0

        if inst[$C_REGISTER$] ~= 0 then pc = pc + 1 end".to_string()
        }
        OpcodeType::OpLoadNil => "for i = inst[$A_REGISTER$], inst[$B_REGISTER$] do memory[i] = nil end".to_string(),
        OpcodeType::OpGetUpval => {
            "local uv = upvals[inst[$B_REGISTER$]]

        memory[inst[$A_REGISTER$]] = uv[2][uv[1]]".to_string()
        }
        OpcodeType::OpGetGlobal => "memory[inst[$A_REGISTER$]] = env[inst[$CONSTANT$]]".to_string(),
        OpcodeType::OpGetTable => "memory[inst[$A_REGISTER$]] = memory[inst[$B_REGISTER$]][constantC(inst)]".to_string(),
        OpcodeType::OpSetGlobal => "env[inst[$CONSTANT$]] = memory[inst[$A_REGISTER$]]".to_string(),
        OpcodeType::OpSetUpval => {
            "local uv = upvals[inst[$B_REGISTER$]]

        uv[2][uv[1]] = memory[inst[$A_REGISTER$]]".to_string()
        }
        OpcodeType::OpSetTable => "memory[inst[$A_REGISTER$]][constantB(inst)] = constantC(inst)".to_string(),
        OpcodeType::OpNewTable => "memory[inst[$A_REGISTER$]] = {}".to_string(),
        OpcodeType::OpSelf => {
            "memory[inst[$A_REGISTER$] + 1] = memory[inst[$B_REGISTER$]]
        memory[inst[$A_REGISTER$]] = memory[inst[$B_REGISTER$]][constantC(inst)]".to_string()
        }
        OpcodeType::OpAdd => "memory[inst[$A_REGISTER$]] = constantB(inst) + constantC(inst)".to_string(),
        OpcodeType::OpSub => "memory[inst[$A_REGISTER$]] = constantB(inst) - constantC(inst)".to_string(),
        OpcodeType::OpMul => "memory[inst[$A_REGISTER$]] = constantB(inst) * constantC(inst)".to_string(),
        OpcodeType::OpDiv => "memory[inst[$A_REGISTER$]] = constantB(inst) / constantC(inst)".to_string(),
        OpcodeType::OpMod => "memory[inst[$A_REGISTER$]] = constantB(inst) % constantC(inst)".to_string(),
        OpcodeType::OpPow => "memory[inst[$A_REGISTER$]] = constantB(inst) ^ constantC(inst)".to_string(),
        OpcodeType::OpUnm => "memory[inst[$A_REGISTER$]] = -memory[inst[$B_REGISTER$]]".to_string(),
        OpcodeType::OpNot => "memory[inst[$A_REGISTER$]] = not memory[inst[$B_REGISTER$]]".to_string(),
        OpcodeType::OpLen => "memory[inst[$A_REGISTER$]] = #memory[inst[$B_REGISTER$]]".to_string(),
        OpcodeType::OpConcat => {
            "local B = inst[$B_REGISTER$]
        local str = memory[B]

        for i = B + 1, inst[$C_REGISTER$] do str = str .. memory[i] end

        memory[inst[$A_REGISTER$]] = str".to_string()
        }
        OpcodeType::OpJmp => "pc = pc + inst[$B_REGISTER$]".to_string(),
        OpcodeType::OpEq => "if (constantB(inst) == constantC(inst)) == (inst[$A_REGISTER$] ~= 0) then pc = pc + code[pc][$B_REGISTER$] end

        pc = pc + 1".to_string(),
        OpcodeType::OpLt => "if (constantB(inst) < constantC(inst)) == (inst[$A_REGISTER$] ~= 0) then pc = pc + code[pc][$B_REGISTER$] end

        pc = pc + 1".to_string(),
        OpcodeType::OpLe => "if (constantB(inst) <= constantC(inst)) == (inst[$A_REGISTER$] ~= 0) then pc = pc + code[pc][$B_REGISTER$] end

        pc = pc + 1".to_string(),
        OpcodeType::OpTest => "if (not memory[inst[$A_REGISTER$]]) ~= (inst[$C_REGISTER$] ~= 0) then pc = pc + code[pc][$B_REGISTER$] end
        pc = pc + 1".to_string(),
        OpcodeType::OpTestSet => "local A = inst[$A_REGISTER$]
        local B = inst[$B_REGISTER$]

        if (not memory[B]) ~= (inst[$C_REGISTER$] ~= 0) then
            memory[A] = memory[B]
            pc = pc + code[pc][$B_REGISTER$]
        end
        pc = pc + 1".to_string(),
        OpcodeType::OpCall => "local A = inst[$A_REGISTER$]
        local B = inst[$B_REGISTER$]
        local C = inst[$C_REGISTER$]
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

        TableMove(ret_list, 1, ret_num, A, memory)".to_string(),
        OpcodeType::OpTailCall => "local A = inst[$A_REGISTER$]
        local B = inst[$B_REGISTER$]
        local params

        if B == 0 then
            params = top_index - A
        else
            params = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return memory[A](TableUnpack(memory, A + 1, A + params))".to_string(),
        OpcodeType::OpReturn => "local A = inst[$A_REGISTER$]
        local B = inst[$B_REGISTER$]
        local len

        if B == 0 then
            len = top_index - A + 1
        else
            len = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return TableUnpack(memory, A, A + len - 1)".to_string(),
        OpcodeType::OpForLoop => "local A = inst[$A_REGISTER$]
        local step = memory[A + 2]
        local index = memory[A] + step
        local limit = memory[A + 1]
        local loops

        if step == MathAbs(step) then
            loops = index <= limit
        else
            loops = index >= limit
        end

        if loops then
            memory[A] = index
            memory[A + 3] = index
            pc = pc + inst[$B_REGISTER$]
        end".to_string(),
        OpcodeType::OpForPrep => "local A = inst[$A_REGISTER$]
        -- local init, limit, step

        -- *: Possible additional error checking
        -- init = assert(tonumber(memory[A]), '`for` initial value must be a number')
        -- limit = assert(tonumber(memory[A + 1]), '`for` limit must be a number')
        -- step = assert(tonumber(memory[A + 2]), '`for` step must be a number')

        local init = Tonumber(memory[A])
        local limit = Tonumber(memory[A + 1])
        local step = Tonumber(memory[A + 2])

        memory[A] = init - step
        memory[A + 1] = limit
        memory[A + 2] = step

        pc = pc + inst[$B_REGISTER$]".to_string(),
        OpcodeType::OpTForLoop => "local A = inst[$A_REGISTER$]
        local base = A + 3

        local vals = {memory[A](memory[A + 1], memory[A + 2])}

        TableMove(vals, 1, inst[$C_REGISTER$], base, memory)

        if memory[base] ~= nil then
            memory[A + 2] = memory[base]
            pc = pc + code[pc][3]
        end

        pc = pc + 1".to_string(),
        OpcodeType::OpSetList => "local A = inst[$A_REGISTER$]
        local C = inst[$C_REGISTER$]
        local len = inst[$B_REGISTER$]
        local tab = memory[A]
        local offset

        if len == 0 then len = top_index - A end

        if C == 0 then
            C = inst[pc][2] -- used to be .value (I think that this is a upvalue but idk so the index might be wrong)
            pc = pc + 1
        end

        offset = (C - 1) * 50 --FIELDS_PER_FLUSH

        TableMove(memory, A + 1, A + len, offset + 1, tab)".to_string(),
        OpcodeType::OpClose => "close_lua_upvalues(open_list, inst[$A_REGISTER$])".to_string(),
        OpcodeType::OpClosure => {
            let mut opcode_string = "local sub = subs[inst[$B_REGISTER$] + 1] -- offset for 1 based index
        local nups = sub[$UPVALUE_COUNT$]
        local uvlist

        if nups ~= 0 then
            uvlist = {}".to_string();

            if opcode_list.contains(&OpcodeType::OpMove) || opcode_list.contains(&OpcodeType::OpGetUpval) {
                opcode_string += "for i = 1, nups do
                local pseudo = code[pc + i - 1]";

                if opcode_list.contains(&OpcodeType::OpMove) {
                    opcode_string += "if pseudo[$OPCODE$] == $MOVE_OPCODE$ then -- @MOVE
                                        uvlist[i - 1] = open_lua_upvalue(open_list, pseudo[$B_REGISTER$], memory)
                                      end";
                }
                
                if opcode_list.contains(&OpcodeType::OpGetUpval) {
                    opcode_string += "if pseudo[$OPCODE$] == $GETUPVAL_OPCODE$ then -- @GETUPVAL
                                        uvlist[i - 1] = upvals[pseudo[$B_REGISTER$]]
                                      end";
                }
                
                opcode_string += " end

            pc = pc + nups";
            }

            

        opcode_string += " end; memory[inst[$A_REGISTER$]] = lua_wrap_state(sub, env, uvlist)";
    
    opcode_string},
        OpcodeType::OpVarArg => "local A = inst[$A_REGISTER$]
        local len = inst[$B_REGISTER$]

        if len == 0 then
            len = vararg.len
            top_index = A + len - 1
        end

        TableMove(vararg.list, 1, len, A, memory)".to_string(),
    }
    
}
