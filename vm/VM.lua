--[[
    VM Stuff

    TODO: Add junk registers between real ones
    TODO: Remove all the Opcode values in the instruction (put instruction index with the registers in numbered values)
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

        local function loop()
            while true do
                local instructionData = chunk.Instructions[pointer]
                pointer = pointer + 1

                --TODO: Implement all of the instructions
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