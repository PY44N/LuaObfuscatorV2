--[[
    VM Stuff

    TODO: Add junk registers between real ones
    TODO: Remove all the named values in the chunk (put randomized index for everything)
]]

local chunkData = {
    ['Constants'] = {
            [1] = 'Hello world',
            [0] = 'print'
    },
    ['Protos'] = {

    },
    ['LastLine'] = 0, -- (DEBUG)
    ['Vargs'] = 2, -- (DEBUG)
    ['FirstLine'] = 0, -- (DEBUG)
    ['Instructions'] = {
            [1] = {
                    [1] = 0,
                    [2] = 0,
                    [3] = false,
                    ['Opcode'] = 5
            },
            [2] = {
                    [1] = 1,
                    [2] = 1,
                    [3] = false,
                    ['Opcode'] = 1
            },
            [3] = {
                    [1] = 0,
                    [2] = 2,
                    [3] = 1,
                    ['Opcode'] = 28
            },
            [4] = {
                    [1] = 0,
                    [2] = 1,
                    [3] = 0,
                    ['Opcode'] = 30
            }
    },
    ['Args'] = 0,
    ['Stack'] = 2,
    ['Upvalues'] = 0,
    ['Lines'] = { -- (DEBUG)
            [1] = 9,
            [2] = 9,
            [3] = 9,
            [4] = 9
    }
}

local Select = select
local String = string
local Char = String.char
local Pcall = pcall
local Nil = nil
local Table = table
local Unpack = Table.unpack
local Getfenv = getfenv

local function run_chunk(chunk)
    return function(...)
        local pointer = 1
        local top = -1

        local varargs = {}
        local varargCount = Select(Char(35), ...) + top -- Using top to subtract 1

        local realStack = {}
        local upvalues = {}
        local stack	= setmetatable({}, {
			__index = realStack;
			__newindex = function(_, key, value)
				if (key > top) then
					top	= key;
				end;

				realStack[key] = value;
			end;
		});

        local function registerOrConstant(register)
            --?: Does checking if register is greater than 256 actually check for register or constant?
            if register >= 256 then
                return register - 256
            end
            return stack[register]
        end

        local function loop()
            while true do
                local instructionData = chunk.Instructions[pointer]
                pointer = pointer + 1

                --TODO: Implement all of the instructions
                local instructionImplementations = {
                    [0] = function() -- Move
                        stack[instructionData[1]] = stack[instructionData[2]]
                    end,
                    [1] = function() -- LoadConst
                        stack[instructionData[1]] = chunk.Constants[instructionData[2]]
                    end,
                    [2] = function() -- LoadBool
                        stack[instructionData[1]] = instructionData[2] ~= 0

                        if instructionData[3] ~= 0 then
                            pointer = pointer + 1
                        end
                    end,
                    [3] = function() -- LoadNil 
                        for i = instructionData[1], instructionData[2] do
                            stack[i] = nil
                        end
                    end,
                    [4] = function() -- GetUpval
                        stack[instructionData[1]] = upvalues[instructionData[2]]
                    end,
                    [5] = function() -- GetGlobal
                        stack[instructionData[1]] = Getfenv(0)[chunk.Constants[instructionData[2]]]
                    end,
                    [6] = function() -- GetTable
                        stack[instructionData[1]] = stack[instructionData[2]][registerOrConstant(instructionData[3])]
                    end,
                    [7] = function() -- SetGlobal
                        Getfenv(0)[chunk.Constants[instructionData[2]]] = stack[instructionData[1]]
                    end,
                    [8] = function() -- SetUpval
                        upvalues[instructionData[2]] = stack[instructionData[1]]
                    end,
                    [9] = function() -- SetTable
                        stack[instructionData[1]][registerOrConstant(instructionData[2])] = registerOrConstant(instructionData[3])
                    end,
                    [10] = function() -- NewTable
                        stack[instructionData[1]] = {}
                    end,
                    [11] = function() -- Self
                        stack[instructionData[1] + 1] = stack[instructionData[2]]
                        stack[instructionData[1]] = stack[instructionData[2]][registerOrConstant(instructionData[3])]
                    end,
                    [12] = function() -- Add
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) + registerOrConstant(instructionData[3])
                    end,
                    [13] = function() -- Sub
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) - registerOrConstant(instructionData[3])
                    end,
                    [14] = function() -- Mul
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) * registerOrConstant(instructionData[3])
                    end,
                    [15] = function() -- Div
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) / registerOrConstant(instructionData[3])
                    end,
                    [16] = function() -- Mod
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) % registerOrConstant(instructionData[3])
                    end,
                    [17] = function() -- Pow
                        stack[instructionData[1]] = registerOrConstant(instructionData[2]) ^ registerOrConstant(instructionData[3])
                    end,
                    [18] = function() -- Unm
                        stack[instructionData[1]] = -stack[instructionData[2]]
                    end,
                    [19] = function() -- Not
                        stack[instructionData[1]] = not stack[instructionData[2]]
                    end,
                    [20] = function() -- Len
                        stack[instructionData[1]] = #stack[instructionData[2]]
                    end,
                    [21] = function() -- Concat
                        stack[instructionData[1]] = stack[instructionData[2]]
                        for i = instructionData[2] + 1, instructionData[3] do
                            stack[instructionData[1]] = stack[instructionData[1]] .. stack[i]
                        end
                    end,
                    [22] = function() -- Jmp
                        pointer = pointer + instructionData[2]
                    end,
                    [23] = function() -- Eq
                        if (registerOrConstant(instructionData[2]) == registerOrConstant(instructionData[3])) ~= instructionData[1] then
                            pointer = pointer + 1
                        end
                    end,
                    [24] = function() -- Lt
                        if (registerOrConstant(instructionData[2]) < registerOrConstant(instructionData[3])) ~= instructionData[1] then
                            pointer = pointer + 1
                        end
                    end,
                    [25] = function() -- Le
                        if (registerOrConstant(instructionData[2]) <= registerOrConstant(instructionData[3])) ~= instructionData[1] then
                            pointer = pointer + 1
                        end
                    end,
                    [26] = function() -- Test
                        -- ?: Does this work?
                        if (instructionData[3] == 0 and stack[instructionData[1]]) or ((not instructionData[3]) and (not stack[instructionData[1]])) then
                            pointer = pointer + 1
                        end
                    end,
                    [27] = function() -- TestSet
                        --?: Does this work
                        if instructionData[3] then 
                            if stack[instructionData[2]] then
                                InstrPoint = InstrPoint + 1;
                            else 
                                stack[instructionData[1]] = stack[instructionData[2]]
                            end
                        elseif stack[instructionData[2]] then
                            stack[instructionData[1]] = stack[instructionData[2]]
                        else 
                            InstrPoint = InstrPoint + 1;
                        end
                    end,
                    [28] = function() -- Call
                        
                    end
                }

                local returnValue = instructionImplementations[instructionData.Opcode]()
                
                -- Handle the stupid cases where it returns something
                if returnValue ~= Nil then
                    return returnValue
                end
            end
        end

        local args = {...}

        for i = 0, varargCount do
            if (i >= chunk.Args) then
                varargs[i - chunk.Args] = args[i + 1]
            else
                stack[i] = args[i + 1]
            end
        end

        local a, b, c = Pcall(loop)

        if a then
            if b and c > 0 then
                return Unpack(b, 1, c)
            else
                return Nil
            end
        else
            --[[
                If not a than an error has occurred and
                b = Error Message
                Use pointer and chunk lines to find error line if debug info included
            ]]
        end
    end
end

run_chunk(chunkData)