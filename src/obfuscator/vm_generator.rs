use std::collections::HashMap;

use lua_deserializer::{
    enums::{chunk_components::ChunkComponents, opcode_type::OpcodeType},
    structs::chunk::Chunk,
};
use rand::{Rng, random_range, seq::SliceRandom};

use crate::{
    obfuscation_settings::ObfuscationSettings,
    obfuscator::{obfuscation_context::ObfuscationContext, utils::index_of},
};

use super::{
    serializer::Serializer,
    vm::{opcode_strings, vm_strings},
};

#[derive(Clone, PartialEq, Eq)]
pub enum ConstantType {
    NIL,
    BOOLEAN,
    NUMBER,
    STRING,
}

fn get_used_opcodes(chunk: &Chunk) -> Vec<OpcodeType> {
    let mut opcodes = vec![];

    for instruction in &chunk.instructions {
        if !opcodes.contains(&instruction.opcode) {
            opcodes.push(instruction.opcode);
        }
    }

    for proto in &chunk.protos {
        for opcode in get_used_opcodes(proto) {
            if !opcodes.contains(&opcode) {
                opcodes.push(opcode);
            }
        }
    }

    opcodes
}

// From: https://rosettacode.org/wiki/LZW_compression#Rust
fn compress(data: Vec<u8>) -> Vec<u32> {
    // Build initial dictionary.
    let mut dictionary: HashMap<Vec<u8>, u32> = (0u32..=255).map(|i| (vec![i as u8], i)).collect();

    let mut w = Vec::new();
    let mut compressed = Vec::new();

    for b in data {
        let mut wc = w.clone();
        wc.push(b);

        if dictionary.contains_key(&wc) {
            w = wc;
        } else {
            // Write w to output.
            compressed.push(dictionary[&w]);

            // wc is a new sequence; add it to the dictionary.
            dictionary.insert(wc, dictionary.len() as u32);
            w.clear();
            w.push(b);
        }
    }

    // Write remaining output if necessary.
    if !w.is_empty() {
        compressed.push(dictionary[&w]);
    }

    compressed
}

static BASE36_CHARS: &str = "+/=~!@#&_|;:<>*^$0123456789abcdefABCDEF";

fn to_base36(value: u64) -> String {
    let mut ret = String::new();
    let mut value: usize = value.try_into().unwrap();

    loop {
        ret.push(
            BASE36_CHARS
                .chars()
                .nth(value % BASE36_CHARS.len())
                .unwrap(),
        );
        value /= BASE36_CHARS.len();

        if value == 0 {
            break;
        }
    }

    ret.chars().rev().collect()
}

pub struct VMGenerator;

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk, settings: ObfuscationSettings) -> String {
        let mut rand = rand::rng();

        let mut opcode_list = get_used_opcodes(&main_chunk);
        opcode_list.shuffle(&mut rand);

        let mut constant_list = [
            ConstantType::NIL,
            ConstantType::BOOLEAN,
            ConstantType::NUMBER,
            ConstantType::STRING,
        ];
        constant_list.shuffle(&mut rand);

        let mut chunk_component_list = [
            ChunkComponents::CONSTANTS,
            ChunkComponents::INSTRUCTIONS,
            ChunkComponents::PROTOS,
        ];
        chunk_component_list.shuffle(&mut rand);

        let mut string_constant_keys = vec![0u8; random_range(3..6)];
        rand.fill(&mut string_constant_keys[..]);

        let obfuscation_context = ObfuscationContext {
            constant_type_map: constant_list,
            opcode_map: opcode_list.clone(),
            chunk_component_map: chunk_component_list,
            string_constant_keys,
        };

        let mut serializer = Serializer::new(obfuscation_context.clone(), settings.clone());
        let bytes = serializer.serialze(main_chunk);

        let bytecode_string: String = if settings.compress_bytecode {
            compress(bytes)
                .into_iter()
                .map(|v| {
                    let byte_str = to_base36(v as u64);
                    to_base36(byte_str.len() as u64) + &byte_str
                })
                .collect()
        } else {
            bytes
                .into_iter()
                .map(|v| String::from("\\") + &v.to_string())
                .collect()
        };

        let mut vm_string = String::new();

        vm_string += vm_strings::VARIABLE_DECLARATION;

        vm_string += "local decodeKeys = {";
        for i in 0..obfuscation_context.string_constant_keys.len() {
            if i != 0 {
                vm_string += ",";
            }
            vm_string += &format!("{}", obfuscation_context.string_constant_keys[i]);
        }
        vm_string += "}\n";

        vm_string += vm_strings::DESERIALIZER;
        vm_string += &format!(
            "
        local function stm_const_list_INLINE(S)
        local len = stm_int64_INLINE(S)
        local list = TableCreate(len)
    
        for i = 1, len do
            local tt = stm_byte_INLINE(S)
            local k
    
            if tt == {} then -- Bool
                k = stm_byte_INLINE(S) ~= 0
            elseif tt == {} then -- Number
                k = stm_num_INLINE(S)
            elseif tt == {} then -- String
                k = decode_INLINE(stm_lstring_INLINE(S))
            end
    
            list[i] = k -- offset +1 during instruction decode
        end
    
        return list
    end
    ",
            index_of(
                &obfuscation_context.constant_type_map,
                ConstantType::BOOLEAN
            ),
            index_of(&obfuscation_context.constant_type_map, ConstantType::NUMBER),
            index_of(&obfuscation_context.constant_type_map, ConstantType::STRING),
        );
        vm_string += vm_strings::DESERIALIZER_2;

        for component in &obfuscation_context.chunk_component_map {
            vm_string += match component {
                ChunkComponents::CONSTANTS => {
                    "proto[$CONSTANT_LIST$] = stm_const_list_INLINE(stream)"
                }
                ChunkComponents::INSTRUCTIONS => {
                    "proto[$OPCODE_LIST$] = stm_inst_list_INLINE(stream)"
                }
                ChunkComponents::PROTOS => "proto[$PROTO_LIST$] = stm_sub_list_INLINE(stream, src)",
            };
            vm_string += "\n";
        }

        if settings.include_debug_line_info {
            vm_string += "proto[$LINE_LIST$] = stm_line_list(stream)";
        }

        vm_string += vm_strings::DESERIALIZER_3;

        vm_string += if settings.include_debug_line_info {
            vm_strings::RUN_HELPERS_LI
        } else {
            vm_strings::RUN_HELPERS
        };
        vm_string += vm_strings::RUN;

        for (i, opcode) in obfuscation_context.opcode_map.iter().enumerate() {
            vm_string += if i == 0 { "if " } else { " elseif " };
            vm_string += &format!("op == {} then --[[{:#?}]] ", i, opcode);
            vm_string += &opcode_strings::get_opcode_string(opcode, &opcode_list);
        }

        vm_string += " end";

        vm_string += if settings.include_debug_line_info {
            vm_strings::RUN_2_LI
        } else {
            vm_strings::RUN_2
        };

        if settings.compress_bytecode {
            vm_string += "
local function RangeGen_INLINE(inputStart, finish, step)
	step = step or 1
	local start = finish and inputStart or 1
	finish = finish or inputStart

	local a = {}

	for i = start, finish, step do
		TableInsert(a, i)
	end

	return a
end

        --local base36Chars = '+/=~!@#&_|;:<>*^$0123456789abcdefABCDEF'
        local base36Chars = StringChar(TableUnpack(TableMerge({43, 47, 61}, RangeGen_INLINE(126, 33, -93), {64}, RangeGen_INLINE(35, 38, 3), {95, 124}, TableReverse_INLINE(RangeGen_INLINE(58, 59)), {60, 62}, RangeGen_INLINE(42, 94, 52), {36}, RangeGen_INLINE(48, 57), RangeGen_INLINE(97, 102), RangeGen_INLINE(65, 70))))

        local function base36Decode_INLINE(inputStr)
            local num, str = 0, StringReverse(inputStr)

            for i = 1, #str do
                num = num + StringFind(base36Chars, StringSub(str, i, i)) * (#base36Chars) ^ (i - 1)
            end

            return num
        end

        -- From https://rosettacode.org/wiki/LZW_compression#Lua
        local function decompress_INLINE(compressed) -- table
            local dictionary, dictSize, entry, w, k = {}, 256, '', StringChar(compressed[1])
            local result = {w}
            for i = 0, 255 do
                dictionary[i] = StringChar(i)
            end
            for i = 2, #compressed do
                k = compressed[i]
                if dictionary[k] then
                    entry = dictionary[k]
                elseif k == dictSize then
                    entry = w .. StringSub(w, 1, 1)
                else
                    return nil, i
                end
                TableInsert(result, entry)
                dictionary[dictSize] = w .. StringSub(entry, 1, 1)
                dictSize = dictSize + 1
                w = entry
            end
            return TableConcat(result)
        end

        local function decode_bytecode_INLINE(bytecode)
            local ret = {}
            local i = 1
            while i <= #bytecode do
                local len = base36Decode_INLINE(StringSub(bytecode, i, i))
                i = i + 1
                TableInsert(ret, base36Decode_INLINE(StringSub(bytecode, i, i + len - 1)))
                i = i + len
            end

            return decompress_INLINE(ret)
        end
        ";
        }

        if settings.compress_bytecode {
            vm_string += &format!(
                "lua_wrap_state(lua_bc_to_state(decode_bytecode_INLINE('{}')))()",
                bytecode_string
            );
        } else {
            vm_string += &format!("lua_wrap_state(lua_bc_to_state('{}'))()", bytecode_string);
        }

        let mut rename_map = [
            "OPCODE",
            "A_REGISTER",
            "B_REGISTER",
            "C_REGISTER",
            "IS_CONST",
            "IS_KB",
            "IS_KC",
            "CONSTANT",
            "CONST_B",
            "CONST_C",
            "SOURCE_NAME",
            "UPVALUE_COUNT",
            "PARAMETER_COUNT",
            "CONSTANT_LIST",
            "OPCODE_LIST",
            "PROTO_LIST",
            "LINE_LIST",
        ];
        rename_map.shuffle(&mut rand);

        for (i, rename) in rename_map.iter().enumerate() {
            vm_string = vm_string.replace(&format!("${}$", *rename), &(i + 1).to_string());
        }

        let move_opcode = opcode_list.iter().position(|&r| r == OpcodeType::OpMove);
        let getupval_opcode = opcode_list
            .iter()
            .position(|&r| r == OpcodeType::OpGetUpval);

        if move_opcode != None {
            vm_string = vm_string.replace("$MOVE_OPCODE$", &move_opcode.unwrap().to_string());
        }

        if getupval_opcode != None {
            vm_string =
                vm_string.replace("$GETUPVAL_OPCODE$", &getupval_opcode.unwrap().to_string());
        }

        vm_string
    }
}
