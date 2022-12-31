use rand::seq::SliceRandom;

use crate::{
    bytecode::{
        enums::{lua_type::LuaType, opcode_type::OpcodeType},
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

fn create_context(const_list: [LuaType; 4], opcode_list: Vec<OpcodeType>) -> ObfuscationContext {
    ObfuscationContext {
        constant_type_map: const_list,
        opcode_map: opcode_list,
    }
}

fn index_of<T>(list: &[T], value: T) -> usize
where
    T: PartialEq<T>,
{
    list.iter().position(|v| *v == value).unwrap()
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

        let obfuscation_context = create_context(constant_list, opcode_list);

        let serializer = Serializer::new(main_chunk);
        let bytes = serializer.serialze(&obfuscation_context, settings);

        let bytecode_string: String = bytes
            .into_iter()
            .map(|v| String::from("\\") + &v.to_string())
            .collect();

        let mut vm_string = String::new();

        vm_string += vm_strings::VARIABLE_DECLARATION;
        vm_string += vm_strings::DESERIALIZER;
        vm_string += &format!(
            "
        local function stm_const_list(S)
        local len = S:int64()
        local list = TableCreate(len)
    
        for i = 1, len do
            local tt = stm_byte(S)
            local k
    
            if tt == {} then -- Bool
                k = stm_byte(S) ~= 0
            elseif tt == {} then -- Number
                k = S:s_num()
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
        vm_string += if settings.include_debug_line_info {
            vm_strings::DESERIALIZER_2_LI
        } else {
            vm_strings::DESERIALIZER_2
        };
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

        vm_string += &format!("lua_wrap_state(lua_bc_to_state('{}'))()", bytecode_string);

        vm_string
    }
}
