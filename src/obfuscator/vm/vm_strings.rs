pub static VARIABLE_DECLARATION: &str = "
local String = string
local StringChar = String.char
local StringByte = String.byte
local StringSub = String.sub
local Select = select
local Table = table
local Math = math
local Error = error
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
";

pub static DESERIALIZER: &str = "
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
local function rd_dbl_le(src, s) return rd_dbl_basic(StringByte(src, s, s + 7)) end

-- byte stm_byte(Stream S)
-- @S - Stream object to read from
local function stm_byte(S)
	local idx = S.index
	local bt = StringByte(S.source, idx, idx)

	S.index = idx + 1
	return bt
end

-- string stm_string(Stream S, int len)
-- @S - Stream object to read from
-- @len - Length of string being read
local function stm_string(S, len)
	local pos = S.index + len
	local str = StringSub(S.source, S.index, pos - 1)

	S.index = pos
	return str
end

-- string stm_lstring(Stream S)
-- @S - Stream object to read from
local function stm_lstring(S)
	local len = S:int64()
	local str

	if len ~= 0 then str = StringSub(stm_string(S, len), 1, -2) end

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

local function stm_sub_list(S, src)
	local len = S:int64()
	local list = TableCreate(len)

	for i = 1, len do
		list[i] = stm_lua_func(S, src) -- offset +1 in CLOSURE
	end

	return list
end
";

pub static DESERIALIZER_2: &str = "
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
";

pub static DESERIALIZER_2_LI: &str = "
local function stm_line_list(S)
	local len = S:int64()
	local list = TableCreate(len)

	for i = 1, len do list[i] = S:int64() end

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
	proto.lines = stm_line_list(stream)

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
";

pub static RUN_HELPERS: &str = "
local function close_lua_upvalues(list, index)
	for i, uv in pairs(list) do
		if uv.index >= index then
			--uv.value = uv.store[uv.index] -- store value
			--uv.store = uv
			--uv.index = 'value' -- self reference
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
	-- TODO: Add line info for optional error reporting
	-- local line = failed.lines[failed.pc - 1]
	local line = 0

	Error(src .. ':' .. line .. ':' .. err, 0)
end
";

pub static RUN_HELPERS_LI: &str = "
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
			--uv.value = uv.store[uv.index] -- store value
			--uv.store = uv
			--uv.index = 'value' -- self reference
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
	-- TODO: Add line info for optional error reporting
	local line = failed.lines[failed.pc - 1]

	Error(src .. ':' .. line .. ':' .. err, 0)
end
";

pub static RUN: &str = "
local function run_lua_func(state, env, upvals)
	local code = state.code
	local subs = state.subs
	local vararg = state.vararg

	local top_index = -1
	local open_list = {}
	local memory = state.memory
	local pc = state.pc

	local function constantB(inst)
		return inst.is_KB and inst.const_B or memory[inst.B]
	end

	local function constantC(inst)
		return inst.is_KC and inst.const_C or memory[inst.C]
	end

	while true do
		local inst = code[pc]
		local op = inst.op
		pc = pc + 1

";

pub static RUN_2: &str = "
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
";

pub static RUN_2_LI: &str = "
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
			local failed = {pc = state.pc, source = proto.source, lines = proto.lines}

			on_lua_error(failed, result[2])

			return
		end
	end

	return wrapped
end
";
