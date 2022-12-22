use crate::{
    bytecode::{
        enums::{lua_type::LuaType, opcode_type::OpcodeType},
        structs::chunk::Chunk,
    },
    obfuscation_settings::ObfuscationSettings,
    obfuscator::obfuscation_context::ObfuscationContext,
};

use super::{serializer::Serializer, vm::vm_strings};

fn create_context() -> ObfuscationContext {
    ObfuscationContext {
        debug_info: true,
        constant_type_map: [
            LuaType::NIL,
            LuaType::BOOLEAN,
            LuaType::NUMBER,
            LuaType::STRING,
        ],
        opcode_map: [
            OpcodeType::OpMove,
            OpcodeType::OpLoadConst,
            OpcodeType::OpLoadBool,
            OpcodeType::OpLoadNil,
            OpcodeType::OpGetUpval,
            OpcodeType::OpGetGlobal,
            OpcodeType::OpGetTable,
            OpcodeType::OpSetGlobal,
            OpcodeType::OpSetUpval,
            OpcodeType::OpSetTable,
            OpcodeType::OpNewTable,
            OpcodeType::OpSelf,
            OpcodeType::OpAdd,
            OpcodeType::OpSub,
            OpcodeType::OpMul,
            OpcodeType::OpDiv,
            OpcodeType::OpMod,
            OpcodeType::OpPow,
            OpcodeType::OpUnm,
            OpcodeType::OpNot,
            OpcodeType::OpLen,
            OpcodeType::OpConcat,
            OpcodeType::OpJmp,
            OpcodeType::OpEq,
            OpcodeType::OpLt,
            OpcodeType::OpLe,
            OpcodeType::OpTest,
            OpcodeType::OpTestSet,
            OpcodeType::OpCall,
            OpcodeType::OpTailCall,
            OpcodeType::OpReturn,
            OpcodeType::OpForLoop,
            OpcodeType::OpForPrep,
            OpcodeType::OpTForLoop,
            OpcodeType::OpSetList,
            OpcodeType::OpClose,
            OpcodeType::OpClosure,
            OpcodeType::OpVarArg,
        ],
    }
}

pub struct VMGenerator;

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk, settings: &ObfuscationSettings) -> String {
        let obfuscation_context = create_context();

        let serializer = Serializer::new(main_chunk, obfuscation_context);
        let bytes = serializer.serialze(settings);

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
        vm_string += if settings.include_debug_line_info {
            vm_strings::RUN_2_LI
        } else {
            vm_strings::RUN_2
        };

        vm_string += &format!("lua_wrap_state(lua_bc_to_state('{}'))()", bytecode_string);

        vm_string
    }
}
