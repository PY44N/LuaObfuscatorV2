use crate::bytecode::enums::opcode_type::OpcodeType;

pub fn get_opcode_string(opcode: &OpcodeType) -> String {
    match opcode {
        OpcodeType::OpMove => "memory[inst.A] = memory[inst.B]",
        OpcodeType::OpLoadConst => "memory[inst.A] = inst.const",
        OpcodeType::OpLoadBool => {
            "memory[inst.A] = inst.B ~= 0

        if inst.C ~= 0 then pc = pc + 1 end"
        }
        OpcodeType::OpLoadNil => "for i = inst.A, inst.B do memory[i] = nil end",
        OpcodeType::OpGetUpval => {
            "local uv = upvals[inst.B]

        memory[inst.A] = uv.store[uv.index]"
        }
        OpcodeType::OpGetGlobal => "memory[inst.A] = env[inst.const]",
        OpcodeType::OpGetTable => "memory[inst.A] = memory[inst.B][constantC(inst)]",
        OpcodeType::OpSetGlobal => "env[inst.const] = memory[inst.A]",
        OpcodeType::OpSetUpval => {
            "local uv = upvals[inst.B]

        uv.store[uv.index] = memory[inst.A]"
        }
        OpcodeType::OpSetTable => "memory[inst.A][constantB(inst)] = constantC(inst)",
        OpcodeType::OpNewTable => "memory[inst.A] = {}",
        OpcodeType::OpSelf => {
            "memory[inst.A + 1] = memory[inst.B]
        memory[inst.A] = memory[inst.B][constantC(inst)]"
        }
        OpcodeType::OpAdd => "memory[inst.A] = constantB(inst) + constantC(inst)",
        OpcodeType::OpSub => "memory[inst.A] = constantB(inst) - constantC(inst)",
        OpcodeType::OpMul => "memory[inst.A] = constantB(inst) * constantC(inst)",
        OpcodeType::OpDiv => "memory[inst.A] = constantB(inst) / constantC(inst)",
        OpcodeType::OpMod => "memory[inst.A] = constantB(inst) % constantC(inst)",
        OpcodeType::OpPow => "memory[inst.A] = constantB(inst) ^ constantC(inst)",
        OpcodeType::OpUnm => "memory[inst.A] = -memory[inst.B]",
        OpcodeType::OpNot => "memory[inst.A] = not memory[inst.B]",
        OpcodeType::OpLen => "memory[inst.A] = #memory[inst.B]",
        OpcodeType::OpConcat => {
            "local B = inst.B
        local str = memory[B]

        for i = B + 1, inst.C do str = str .. memory[i] end

        memory[inst.A] = str"
        }
        OpcodeType::OpJmp => "pc = pc + inst.sBx",
        OpcodeType::OpEq => "if (constantB(inst) == constantC(inst)) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

        pc = pc + 1",
        OpcodeType::OpLt => "if (constantB(inst) < constantC(inst)) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

        pc = pc + 1",
        OpcodeType::OpLe => "if (constantB(inst) <= constantC(inst)) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

        pc = pc + 1",
        OpcodeType::OpTest => "if (not memory[inst.A]) ~= (inst.C ~= 0) then pc = pc + code[pc].sBx end
        pc = pc + 1",
        OpcodeType::OpTestSet => "local A = inst.A
        local B = inst.B

        if (not memory[B]) ~= (inst.C ~= 0) then
            memory[A] = memory[B]
            pc = pc + code[pc].sBx
        end
        pc = pc + 1",
        OpcodeType::OpCall => "local A = inst.A
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

        TableMove(ret_list, 1, ret_num, A, memory)",
        OpcodeType::OpTailCall => "local A = inst.A
        local B = inst.B
        local params

        if B == 0 then
            params = top_index - A
        else
            params = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return memory[A](TableUnpack(memory, A + 1, A + params))",
        OpcodeType::OpReturn => "local A = inst.A
        local B = inst.B
        local len

        if B == 0 then
            len = top_index - A + 1
        else
            len = B - 1
        end

        close_lua_upvalues(open_list, 0)

        return TableUnpack(memory, A, A + len - 1)",
        OpcodeType::OpForLoop => "local A = inst.A
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
            pc = pc + inst.sBx
        end",
        OpcodeType::OpForPrep => "local A = inst.A
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

        pc = pc + inst.sBx",
        OpcodeType::OpTForLoop => "local A = inst.A
        local base = A + 3

        local vals = {memory[A](memory[A + 1], memory[A + 2])}

        TableMove(vals, 1, inst.C, base, memory)

        if memory[base] ~= nil then
            memory[A + 2] = memory[base]
            pc = pc + code[pc].sBx
        end

        pc = pc + 1",
        OpcodeType::OpSetList => "local A = inst.A
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

        TableMove(memory, A + 1, A + len, offset + 1, tab)",
        OpcodeType::OpClose => "close_lua_upvalues(open_list, inst.A)",
        OpcodeType::OpClosure => "local sub = subs[inst.Bx + 1] -- offset for 1 based index
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

        memory[inst.A] = lua_wrap_state(sub, env, uvlist)",
        OpcodeType::OpVarArg => "local A = inst.A
        local len = inst.B

        if len == 0 then
            len = vararg.len
            top_index = A + len - 1
        end

        TableMove(vararg.list, 1, len, A, memory)",
    }
    .to_string()
}
