use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::{
    bytecode::{
        enums::{chunk_components::ChunkComponents, lua_type::LuaType, opcode_type::OpcodeType},
        structs::chunk::Chunk,
    },
    obfuscation_settings::ObfuscationSettings,
    obfuscator::obfuscation_context::ObfuscationContext,
};

use super::{
    serializer::Serializer,
    vm::{opcode_strings, vm_strings},
};

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

fn create_context(
    const_list: [LuaType; 4],
    opcode_list: Vec<OpcodeType>,
    chunk_component_list: [ChunkComponents; 3],
) -> ObfuscationContext {
    ObfuscationContext {
        constant_type_map: const_list,
        opcode_map: opcode_list,
        chunk_component_map: chunk_component_list,
    }
}

fn index_of<T>(list: &[T], value: T) -> usize
where
    T: PartialEq<T>,
{
    list.iter().position(|v| *v == value).unwrap()
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

static BASE64_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn to_base64(value: u64) -> String {
    let mut ret = String::new();
    let mut value: usize = value.try_into().unwrap();

    loop {
        ret.push(
            BASE64_CHARS
                .chars()
                .nth(value % BASE64_CHARS.len())
                .unwrap(),
        );
        value /= BASE64_CHARS.len();

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

    pub fn generate(&self, main_chunk: Chunk, settings: &ObfuscationSettings) -> String {
        let mut rand = rand::thread_rng();

        let mut opcode_list = get_used_opcodes(&main_chunk);
        opcode_list.shuffle(&mut rand);

        let mut constant_list = [
            LuaType::NIL,
            LuaType::BOOLEAN,
            LuaType::NUMBER,
            LuaType::STRING,
        ];
        constant_list.shuffle(&mut rand);

        let mut chunk_component_list = [
            ChunkComponents::CONSTANTS,
            ChunkComponents::INSTRUCTIONS,
            ChunkComponents::PROTOS,
        ];
        chunk_component_list.shuffle(&mut rand);

        let obfuscation_context = create_context(constant_list, opcode_list, chunk_component_list);

        let serializer = Serializer::new(main_chunk);
        let bytes = serializer.serialze(&obfuscation_context, settings);

        let bytecode_string: String = if settings.compress_bytecode {
            compress(bytes)
                .into_iter()
                .map(|v| {
                    let byte_str = to_base64(v as u64);
                    to_base64(byte_str.len() as u64) + &byte_str
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
        vm_string += vm_strings::DESERIALIZER;
        vm_string += &format!(
            "
        local function stm_const_list(S)
        local len = stm_int64(S)
        local list = TableCreate(len)
    
        for i = 1, len do
            local tt = stm_byte(S)
            local k
    
            if tt == {} then -- Bool
                k = stm_byte(S) ~= 0
            elseif tt == {} then -- Number
                k = stm_num(S)
            elseif tt == {} then -- String
                k = stm_lstring(S)
            end
    
            list[i] = k -- offset +1 during instruction decode
        end
    
        return list
    end
    ",
            index_of(&obfuscation_context.constant_type_map, LuaType::BOOLEAN),
            index_of(&obfuscation_context.constant_type_map, LuaType::NUMBER),
            index_of(&obfuscation_context.constant_type_map, LuaType::STRING),
        );
        vm_string += vm_strings::DESERIALIZER_2;

        for component in &obfuscation_context.chunk_component_map {
            vm_string += match component {
                ChunkComponents::CONSTANTS => "proto[$CONSTANT_LIST$] = stm_const_list(stream)",
                ChunkComponents::INSTRUCTIONS => "proto[$OPCODE_LIST$] = stm_inst_list(stream)",
                ChunkComponents::PROTOS => "proto[$PROTO_LIST$] = stm_sub_list(stream, src)",
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
            vm_string += &opcode_strings::get_opcode_string(opcode);
        }

        vm_string += " end";

        vm_string += if settings.include_debug_line_info {
            vm_strings::RUN_2_LI
        } else {
            vm_strings::RUN_2
        };

        if settings.compress_bytecode {
            vm_string += "
        local base36Chars = StringChar(TableUnpack(TableMerge(RangeGen(48, 57), RangeGen(65, 90))))

        local function base36Decode(inputStr)
            local num, str = 0, StringReverse(inputStr)

            for i = 1, #str do
                num = num + StringFind(base36Chars, StringSub(str, i, i)) * 36 ^ (i - 1)
            end

            return num
        end

        -- From https://rosettacode.org/wiki/LZW_compression#Lua
        local function decompress(compressed) -- table
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

        local function decode(bytecode)
            local ret = {}
            local i = 1
            while i <= #bytecode do
                local len = base36Decode(StringSub(bytecode, i, i))
                i = i + 1
                TableInsert(ret, base36Decode(StringSub(bytecode, i, i + len - 1)))
                i = i + len
            end

            return decompress(ret)
        end
        ";
        }

        if settings.compress_bytecode {
            vm_string += &format!(
                "lua_wrap_state(lua_bc_to_state(decode('{}')))()",
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

        vm_string
    }
}
