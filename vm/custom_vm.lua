-- Based on FiOne https://github.com/Rerumu/FiOne/blob/master/source.lua

local String = string
local StringChar = String.char
local StringByte = String.byte
local Select = select
local Table = table
local Math = math
local TableCreate = function(...)
	return {}
end
local TableUnpack = Table.unpack or unpack
local TablePack = function(...)
	return { n = Select(StringChar(35), ...), ... }
end
local TableMove = function(src, first, last, offset, dst)
	for i = 0, last - first do
		dst[offset + i] = src[first + i]
	end
end
local TableConcat = Table.concat
local Getfenv = getfenv
local MathFloor = Math.floor
local MathMax = Math.max
local Pcall = pcall
local MathAbs = Math.abs
local Tonumber = tonumber
local BitAnd, BitRShift, BitLShift = (function()
	local function tobittable_r(x, ...)
		if (x or 0) == 0 then
			return ...
		end
		return tobittable_r(MathFloor(x / 2), x % 2, ...)
	end

	local function tobittable(x)
		if x == 0 then
			return { 0 }
		end
		return { tobittable_r(x) }
	end

	local function makeop(cond)
		local function oper(x, y, ...)
			if not y then
				return x
			end
			x, y = tobittable(x), tobittable(y)
			local xl, yl = #x, #y
			local t, tl = {}, MathMax(xl, yl)
			for i = 0, tl - 1 do
				local b1, b2 = x[xl - i], y[yl - i]
				if not (b1 or b2) then
					break
				end
				t[tl - i] = (cond((b1 or 0) ~= 0, (b2 or 0) ~= 0) and 1 or 0)
			end
			return oper(Tonumber(TableConcat(t), 2), ...)
		end
		return oper
	end

	---
	-- Perform bitwise AND of several numbers.
	-- Truth table:
	--   band(0,0) -> 0,
	--   band(0,1) -> 0,
	--   band(1,0) -> 0,
	--   band(1,1) -> 1.
	-- @class function
	-- @name band
	-- @param ...  Numbers.
	-- @return  A number.
	local band = makeop(function(a, b)
		return a and b
	end)

	---
	-- Shift a number's bits to the left.
	-- Roughly equivalent to (x * (2^bits)).
	-- @param x  The number to shift (number).
	-- @param bits  Number of positions to shift by (number).
	-- @return  A number.
	local function blshift(x, bits)
		return MathFloor(x) * (2 ^ bits)
	end

	---
	-- Shift a number's bits to the right.
	-- Roughly equivalent to (x / (2^bits)).
	-- @param x  The number to shift (number).
	-- @param bits  Number of positions to shift by (number).
	-- @return  A number.
	local function brshift(x, bits)
		return MathFloor(MathFloor(x) / (2 ^ bits))
	end

	return band, brshift, blshift
end)()

local lua_bc_to_state
local lua_wrap_state
local stm_lua_func

-- int rd_int_basic(string src, int s, int e, int d)
-- @src - Source binary string
-- @s - Start index of a little endian integer
-- @e - End index of the integer
-- @d - Direction of the loop
local function rd_int_basic(src, s, e, d)
	local num = 0

	-- if bb[l] > 127 then -- signed negative
	-- 	num = num - 256 ^ l
	-- 	bb[l] = bb[l] - 128
	-- end

	for i = s, e, d do
		local mul = 256 ^ MathAbs(i - s)

		num = num + mul * StringByte(src, i, i)
	end

	return num
end

-- double rd_dbl_basic(byte f1..8)
-- @f1..8 - The 8 bytes composing a little endian double
local function rd_dbl_basic(f1, f2, f3, f4, f5, f6, f7, f8)
	local sign = (-1) ^ BitRShift(f8, 7)
	local exp = BitLShift(BitAnd(f8, 0x7F), 4) + BitRShift(f7, 4)
	local frac = BitAnd(f7, 0x0F) * 2 ^ 48
	local normal = 1

	frac = frac + (f6 * 2 ^ 40) + (f5 * 2 ^ 32) + (f4 * 2 ^ 24) + (f3 * 2 ^ 16) + (f2 * 2 ^ 8) + f1 -- help

	if exp == 0 then
		if frac == 0 then
			return sign * 0
		else
			normal = 0
			exp = 1
		end
	elseif exp == 0x7FF then
		if frac == 0 then
			return sign * (1 / 0)
		else
			return sign * (0 / 0)
		end
	end

	return sign * 2 ^ (exp - 1023) * (normal + frac / 2 ^ 52)
end

-- int rd_int_le(string src, int s, int e)
-- @src - Source binary string
-- @s - Start index of a little endian integer
-- @e - End index of the integer
local function rd_int_le(src, s, e) return rd_int_basic(src, s, e - 1, 1) end

-- double rd_dbl_le(string src, int s)
-- @src - Source binary string
-- @s - Start index of little endian double
local function rd_dbl_le(src, s) return rd_dbl_basic(string.byte(src, s, s + 7)) end

-- byte stm_byte(Stream S)
-- @S - Stream object to read from
local function stm_byte(S)
	local idx = S.index
	local bt = string.byte(S.source, idx, idx)

	S.index = idx + 1
	return bt
end

-- string stm_string(Stream S, int len)
-- @S - Stream object to read from
-- @len - Length of string being read
local function stm_string(S, len)
	local pos = S.index + len
	local str = string.sub(S.source, S.index, pos - 1)

	S.index = pos
	return str
end

-- string stm_lstring(Stream S)
-- @S - Stream object to read from
local function stm_lstring(S)
	local len = S:int64()
	local str

	if len ~= 0 then str = string.sub(stm_string(S, len), 1, -2) end

	return str
end

-- fn cst_int_rdr(string src, int len, fn func)
-- @len - Length of type for reader
-- @func - Reader callback
local function cst_int_rdr(len, func)
	return function(S)
		local pos = S.index + len
		local int = func(S.source, S.index, pos)
		S.index = pos

		return int
	end
end

-- fn cst_flt_rdr(string src, int len, fn func)
-- @len - Length of type for reader
-- @func - Reader callback
local function cst_flt_rdr(len, func)
	return function(S)
		local flt = func(S.source, S.index)
		S.index = S.index + len

		return flt
	end
end

local function stm_inst_list(S)
	local len = S:int64()
	local list = TableCreate(len)

	for i = 1, len do
		-- local ins = S:int16()
		local op = stm_byte(S)
		local args = stm_byte(S)
		local isConstantB = stm_byte(S) == 1
		local isConstantC = stm_byte(S) == 1
		local data = {op = op, A = stm_byte(S)}

		if args == 1 then -- ABC
			data.B = S:int16()
			data.C = S:int16()
			data.is_KB = isConstantB and data.B > 0xFF -- post process optimization
			data.is_KC = isConstantC and data.C > 0xFF
		elseif args == 2 then -- ABx
			data.Bx = S:int32()
			data.is_K = isConstantB
		elseif args == 3 then -- AsBx
			data.sBx = S:int32() - 131071
		end

		list[i] = data
	end


	return list
end

local function stm_const_list(S)
	local len = S:int64()
	local list = TableCreate(len)

	for i = 1, len do
		local tt = stm_byte(S)
		local k

		if tt == 1 then
			k = stm_byte(S) ~= 0
		elseif tt == 2 then
			k = S:s_num()
		elseif tt == 3 then
			k = stm_lstring(S)
		end

		list[i] = k -- offset +1 during instruction decode
	end

	return list
end

local function stm_sub_list(S, src)
	local len = S:int64()
	local list = TableCreate(len)

	for i = 1, len do
		list[i] = stm_lua_func(S, src) -- offset +1 in CLOSURE
	end

	return list
end

local function stm_line_list(S)
	local len = S:s_int()
	local list = TableCreate(len)

	for i = 1, len do list[i] = S:s_int() end

	return list
end

function stm_lua_func(stream, psrc)
	local proto = {}
	local src = stm_lstring(stream) or psrc -- source is propagated

	proto.source = src -- source name

	-- stream:s_int() -- line defined
	-- stream:s_int() -- last line defined

	proto.num_upval = stm_byte(stream) -- num upvalues
	proto.num_param = stm_byte(stream) -- num params


	-- stm_byte(stream) -- vararg flag
	-- proto.max_stack = stm_byte(stream) -- max stack size

	proto.const = stm_const_list(stream)
	proto.code = stm_inst_list(stream)
	proto.subs = stm_sub_list(stream, src)
	-- proto.lines = stm_line_list(stream)

	-- post process optimization
	for _, v in ipairs(proto.code) do
		if v.is_K then
			v.const = proto.const[v.Bx + 1] -- offset for 1 based index
		else
			if v.is_KB then v.const_B = proto.const[v.B - 0xFF] end

			if v.is_KC then v.const_C = proto.const[v.C - 0xFF] end
		end
	end

	return proto
end

function lua_bc_to_state(src)
	-- stream object
	local stream = {
		-- data
		index = 1,
		source = src,
		int16 = cst_int_rdr(2, rd_int_le),
		int32 = cst_int_rdr(4, rd_int_le),
		int64 = cst_int_rdr(8, rd_int_le),
		s_num = cst_flt_rdr(8, rd_dbl_le)
	}

	return stm_lua_func(stream, '@virtual')
end

local function close_lua_upvalues(list, index)
	for i, uv in pairs(list) do
		if uv.index >= index then
			uv.value = uv.store[uv.index] -- store value
			uv.store = uv
			uv.index = 'value' -- self reference
			list[i] = nil
		end
	end
end

local function open_lua_upvalue(list, index, memory)
	local prev = list[index]

	if not prev then
		prev = {index = index, store = memory}
		list[index] = prev
	end

	return prev
end

local function on_lua_error(failed, err)
	local src = failed.source
	-- local line = failed.lines[failed.pc - 1]
	local line = 0

	error(string.format('%s:%i: %s', src, line, err), 0)
end

local function run_lua_func(state, env, upvals)
	local code = state.code
	local subs = state.subs
	local vararg = state.vararg

	local top_index = -1
	local open_list = {}
	local memory = state.memory
	local pc = state.pc

	while true do
		local inst = code[pc]
		local op = inst.op
		pc = pc + 1

		if op == 0 then
			--[[MOVE]]
			memory[inst.A] = memory[inst.B]
		elseif op == 1 then
			--[[LOADK]]
			memory[inst.A] = inst.const
		elseif op == 2 then
			--[[LOADBOOL]]
			memory[inst.A] = inst.B ~= 0

			if inst.C ~= 0 then pc = pc + 1 end
		elseif op == 3 then
			--[[LOADNIL]]
			for i = inst.A, inst.B do memory[i] = nil end
		elseif op == 4 then
			--[[GETUPVAL]]
			local uv = upvals[inst.B]

			memory[inst.A] = uv.store[uv.index]
		elseif op == 5 then
			--[[GETGLOBAL]]
			memory[inst.A] = env[inst.const]
		elseif op == 6 then
			--[[GETTABLE]]
			local index

			if inst.is_KC then
				index = inst.const_C
			else
				index = memory[inst.C]
			end

			memory[inst.A] = memory[inst.B][index]
		elseif op == 7 then
			--[[SETGLOBAL]]
			env[inst.const] = memory[inst.A]
		elseif op == 8 then
			--[[SETUPVAL]]
			local uv = upvals[inst.B]

			uv.store[uv.index] = memory[inst.A]
		elseif op == 9 then
			--[[SETTABLE]]
			local index, value

			if inst.is_KB then
				index = inst.const_B
			else
				index = memory[inst.B]
			end

			if inst.is_KC then
				value = inst.const_C
			else
				value = memory[inst.C]
			end

			memory[inst.A][index] = value
		elseif op == 10 then
			--[[NEWTABLE]]
			memory[inst.A] = {}
		elseif op == 11 then
			--[[SELF]]
			local A = inst.A
			local B = inst.B
			local index

			if inst.is_KC then
				index = inst.const_C
			else
				index = memory[inst.C]
			end

			memory[A + 1] = memory[B]
			memory[A] = memory[B][index]
		elseif op == 12 then
			--[[ADD]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs + rhs
		elseif op == 13 then
			--[[SUB]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs - rhs
		elseif op == 14 then
			--[[MUL]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs * rhs
		elseif op == 15 then
			--[[DIV]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs / rhs
		elseif op == 16 then
			--[[MOD]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs % rhs
		elseif op == 17 then
			--[[POW]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			memory[inst.A] = lhs ^ rhs
		elseif op == 18 then
			--[[UNM]]
			memory[inst.A] = -memory[inst.B]
		elseif op == 19 then
			--[[NOT]]
			memory[inst.A] = not memory[inst.B]
		elseif op == 20 then
			--[[LEN]]
			memory[inst.A] = #memory[inst.B]
		elseif op == 21 then
			--[[CONCAT]]
			local B = inst.B
			local str = memory[B]

			for i = B + 1, inst.C do str = str .. memory[i] end

			memory[inst.A] = str
		elseif op == 22 then
			--[[JMP]]
			pc = pc + inst.sBx
		elseif op == 23 then
			--[[EQ]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			if (lhs == rhs) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

			pc = pc + 1
		elseif op == 24 then
			--[[LT]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			if (lhs < rhs) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

			pc = pc + 1
		elseif op == 25 then
			--[[LE]]
			local lhs, rhs

			if inst.is_KB then
				lhs = inst.const_B
			else
				lhs = memory[inst.B]
			end

			if inst.is_KC then
				rhs = inst.const_C
			else
				rhs = memory[inst.C]
			end

			if (lhs <= rhs) == (inst.A ~= 0) then pc = pc + code[pc].sBx end

			pc = pc + 1
		elseif op == 26 then
			--[[TEST]]
			if (not memory[inst.A]) ~= (inst.C ~= 0) then pc = pc + code[pc].sBx end
			pc = pc + 1
		elseif op == 27 then
			--[[TESTSET]]
			local A = inst.A
			local B = inst.B

			if (not memory[B]) ~= (inst.C ~= 0) then
				memory[A] = memory[B]
				pc = pc + code[pc].sBx
			end
			pc = pc + 1
		elseif op == 28 then
			--[[CALL]]
			local A = inst.A
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

			TableMove(ret_list, 1, ret_num, A, memory)
		elseif op == 29 then
			--[[TAILCALL]]
			local A = inst.A
			local B = inst.B
			local params

			if B == 0 then
				params = top_index - A
			else
				params = B - 1
			end

			close_lua_upvalues(open_list, 0)

			return memory[A](TableUnpack(memory, A + 1, A + params))
		elseif op == 30 then
			--[[RETURN]]
			local A = inst.A
			local B = inst.B
			local len

			if B == 0 then
				len = top_index - A + 1
			else
				len = B - 1
			end

			close_lua_upvalues(open_list, 0)

			return TableUnpack(memory, A, A + len - 1)
		elseif op == 31 then
			--[[FORLOOP]]
			local A = inst.A
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
			end
		elseif op == 32 then
			--[[FORPREP]]
			local A = inst.A
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

			pc = pc + inst.sBx
		elseif op == 33 then
			--[[TFORLOOP]]
			local A = inst.A
			local base = A + 3

			local vals = {memory[A](memory[A + 1], memory[A + 2])}

			TableMove(vals, 1, inst.C, base, memory)

			if memory[base] ~= nil then
				memory[A + 2] = memory[base]
				pc = pc + code[pc].sBx
			end

			pc = pc + 1
		elseif op == 34 then
			--[[SETLIST]]
			local A = inst.A
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

			TableMove(memory, A + 1, A + len, offset + 1, tab)
		elseif op == 35 then
			--[[CLOSE]]
			close_lua_upvalues(open_list, inst.A)
		elseif op == 36 then
			--[[CLOSURE]]
			local sub = subs[inst.Bx + 1] -- offset for 1 based index
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

			memory[inst.A] = lua_wrap_state(sub, env, uvlist)
		elseif op == 37 then
			--[[VARARG]]
			local A = inst.A
			local len = inst.B

			if len == 0 then
				len = vararg.len
				top_index = A + len - 1
			end

			TableMove(vararg.list, 1, len, A, memory)
		end

		state.pc = pc
	end
end

function lua_wrap_state(proto, env, upval)
	env = env or Getfenv(0)

	local function wrapped(...)
		local passed = TablePack(...)
		local memory = TableCreate()
		local vararg = {len = 0, list = {}}

		TableMove(passed, 1, proto.num_param, 0, memory)

		if proto.num_param < passed.n then
			local start = proto.num_param + 1
			local len = passed.n - proto.num_param

			vararg.len = len
			TableMove(passed, start, start + len - 1, 1, vararg.list)
		end

		local state = {vararg = vararg, memory = memory, code = proto.code, subs = proto.subs, pc = 1}

		local result = TablePack(Pcall(run_lua_func, state, env, upval))

		if result[1] then
			return TableUnpack(result, 2, result.n)
		else
			local failed = {pc = state.pc, source = proto.source --[[,lines = proto.lines]]}

			on_lua_error(failed, result[2])

			return
		end
	end

	return wrapped
end

local bytecode = "\11\0\0\0\0\0\0\0\64\116\101\109\112\49\46\108\117\97\0\0\0\3\0\0\0\0\0\0\0\3\6\0\0\0\0\0\0\0\72\101\108\108\111\0\3\6\0\0\0\0\0\0\0\119\111\114\108\100\0\2\0\0\0\0\0\0\240\63\15\0\0\0\0\0\0\0\10\1\0\0\0\2\0\0\0\1\2\1\0\1\0\0\0\0\1\2\1\0\2\1\0\0\0\34\1\0\0\0\2\0\1\0\36\2\0\0\1\0\0\0\0\1\2\1\0\2\2\0\0\0\20\1\0\0\3\0\0\0\0\25\1\1\1\0\2\0\3\0\22\3\0\0\0\4\0\2\0\0\1\0\0\3\1\0\0\0\6\1\0\1\4\0\0\2\0\28\1\0\0\3\2\0\1\0\12\1\1\1\2\2\0\2\1\22\3\0\0\0\247\255\1\0\30\1\0\0\0\1\0\0\0\1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\1\0\0\0\0\0\0\0\3\6\0\0\0\0\0\0\0\112\114\105\110\116\0\4\0\0\0\0\0\0\0\5\2\1\0\1\0\0\0\0\37\1\0\0\2\0\0\0\0\28\1\0\0\1\0\0\1\0\30\1\0\0\0\1\0\0\0\0\0\0\0\0\0\0\0"

lua_wrap_state(lua_bc_to_state(bytecode))()

