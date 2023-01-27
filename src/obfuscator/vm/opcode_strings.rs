use crate::bytecode::enums::opcode_type::OpcodeType;

pub fn get_opcode_string(opcode: &OpcodeType) -> String {
    match opcode {
        OpcodeType::OpMove => "memory[inst[2]] = memory[inst[3]]",
        OpcodeType::OpLoadConst => "memory[inst[2]] = inst[8]",
        OpcodeType::OpLoadBool => {
            "memory[inst[2]] = inst[3] ~= 0

        if inst[4] ~= 0 then pc = pc + 1 end"
        }
        OpcodeType::OpLoadNil => "for i = inst[2], inst[3] do memory[i] = nil end",
        OpcodeType::OpGetUpval => {
            "local uv = upvals[inst[3]]

        memory[inst[2]] = uv[2][uv.index]"
        }
        OpcodeType::OpGetGlobal => "memory[inst[2]] = env[inst[8]]",
        OpcodeType::OpGetTable => "memory[inst[2]] = memory[inst[3]][constantC(inst)]",
        OpcodeType::OpSetGlobal => "env[inst[8]] = memory[inst[2]]",
        OpcodeType::OpSetUpval => {
            "local uv = upvals[inst[3]]

        uv[2][uv[1]] = memory[inst[2]]"
        }
        OpcodeType::OpSetTable => "memory[inst[2]][constantB(inst)] = constantC(inst)",
        OpcodeType::OpNewTable => "memory[inst[2]] = {}",
        OpcodeType::OpSelf => {
            "memory[inst[2] + 1] = memory[inst[3]]
        memory[inst[2]] = memory[inst[3]][constantC(inst)]"
        }
        OpcodeType::OpAdd => "memory[inst[2]] = constantB(inst) + constantC(inst)",
        OpcodeType::OpSub => "memory[inst[2]] = constantB(inst) - constantC(inst)",
        OpcodeType::OpMul => "memory[inst[2]] = constantB(inst) * constantC(inst)",
        OpcodeType::OpDiv => "memory[inst[2]] = constantB(inst) / constantC(inst)",
        OpcodeType::OpMod => "memory[inst[2]] = constantB(inst) % constantC(inst)",
        OpcodeType::OpPow => "memory[inst[2]] = constantB(inst) ^ constantC(inst)",
        OpcodeType::OpUnm => "memory[inst[2]] = -memory[inst[3]]",
        OpcodeType::OpNot => "memory[inst[2]] = not memory[inst[3]]",
        OpcodeType::OpLen => "memory[inst[2]] = #memory[inst[3]]",
        OpcodeType::OpConcat => {
            "local B = inst[3]
        local str = memory[B]

        for i = B + 1, inst[4] do str = str .. memory[i] end

        memory[inst[2]] = str"
        }
        OpcodeType::OpJmp => "pc = pc + inst[3]",
        OpcodeType::OpEq => "if (constantB(inst) == constantC(inst)) == (inst[2] ~= 0) then pc = pc + code[pc][3] end

        pc = pc + 1",
        OpcodeType::OpLt => "if (constantB(inst) < constantC(inst)) == (inst[2] ~= 0) then pc = pc + code[pc][3] end

        pc = pc + 1",
        OpcodeType::OpLe => "if (constantB(inst) <= constantC(inst)) == (inst[2] ~= 0) then pc = pc + code[pc][3] end

        pc = pc + 1",
        OpcodeType::OpTest => "if (not memory[inst[2]]) ~= (inst[4] ~= 0) then pc = pc + code[pc][3] end
        pc = pc + 1",
        OpcodeType::OpTestSet => "local A = inst[2]
        local B = inst[3]

        if (not memory[B]) ~= (inst[4] ~= 0) then
            memory[A] = memory[B]
            pc = pc + code[pc][3]
        end
        pc = pc + 1",
        OpcodeType::OpCall => "local A = inst[2]
        local B = inst[3]
        local C = inst[4]
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

        TableMove(ret_list, 1, ret_num, A, memory)",
        OpcodeType::OpTailCall => "local A = inst[2]
        local B = inst[3]
        local params

        if B == 0 then
            params = top_index - A
        else
            params = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return memory[A](TableUnpack(memory, A + 1, A + params))",
        OpcodeType::OpReturn => "local A = inst[2]
        local B = inst[3]
        local len

        if B == 0 then
            len = top_index - A + 1
        else
            len = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return TableUnpack(memory, A, A + len - 1)",
        OpcodeType::OpForLoop => "local A = inst[2]
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
            pc = pc + inst[3]
        end",
        OpcodeType::OpForPrep => "local A = inst[2]
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

        pc = pc + inst[3]",
        OpcodeType::OpTForLoop => "local A = inst[2]
        local base = A + 3

        local vals = {memory[A](memory[A + 1], memory[A + 2])}

        TableMove(vals, 1, inst[4], base, memory)

        if memory[base] ~= nil then
            memory[A + 2] = memory[base]
            pc = pc + code[pc][3]
        end

        pc = pc + 1",
        OpcodeType::OpSetList => "local A = inst[2]
        local C = inst[4]
        local len = inst[3]
        local tab = memory[A]
        local offset

        if len == 0 then len = top_index - A end

        if C == 0 then
            C = inst[pc][2] -- used to be .value (I think that this is a upvalue but idk so the index might be wrong)
            pc = pc + 1
        end

        offset = (C - 1) * 50 --FIELDS_PER_FLUSH

        TableMove(memory, A + 1, A + len, offset + 1, tab)",
        OpcodeType::OpClose => "close_lua_upvalues(open_list, inst[2])",
        OpcodeType::OpClosure => "local sub = subs[inst[3] + 1] -- offset for 1 based index
        local nups = sub[2]
        local uvlist

        if nups ~= 0 then
            uvlist = {}

            for i = 1, nups do
                local pseudo = code[pc + i - 1]

                if pseudo.op == 0 then -- @MOVE
                    uvlist[i - 1] = open_lua_upvalue(open_list, pseudo[3], memory)
                elseif pseudo.op == 4 then -- @GETUPVAL
                    uvlist[i - 1] = upvals[pseudo[3]]
                end
            end

            pc = pc + nups
        end

        memory[inst[2]] = lua_wrap_state(sub, env, uvlist)",
        OpcodeType::OpVarArg => "local A = inst[2]
        local len = inst[3]

        if len == 0 then
            len = vararg.len
            top_index = A + len - 1
        end

        TableMove(vararg.list, 1, len, A, memory)",
    }
    .to_string()
}
