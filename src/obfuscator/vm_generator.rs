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

fn create_context(opcode_list: Vec<OpcodeType>) -> ObfuscationContext {
    ObfuscationContext {
        constant_type_map: [
            LuaType::NIL,
            LuaType::BOOLEAN,
            LuaType::NUMBER,
            LuaType::STRING,
        ],
        opcode_map: opcode_list,
    }
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

        let obfuscation_context = create_context(opcode_list);

        let serializer = Serializer::new(main_chunk);
        let bytes = serializer.serialze(&obfuscation_context, settings);

        let bytecode_string: String = bytes
            .into_iter()
            .map(|v| String::from("\\") + &v.to_string())
            .collect();

        let mut vm_string = String::new();

        vm_string += vm_strings::VARIABLE_DECLARATION;
        vm_string += vm_strings::DESERIALIZER;
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
