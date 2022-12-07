use crate::{
    bytecode::{enums::lua_type::LuaType, structs::chunk::Chunk},
    obfuscator::obfuscation_context::ObfuscationContext,
};

use super::serializer::Serializer;

pub struct VMGenerator {}

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk) {
        let obfuscation_context = ObfuscationContext {
            debug_info: true,
            constant_type_map: [
                LuaType::NIL,
                LuaType::BOOLEAN,
                LuaType::NUMBER,
                LuaType::STRING,
            ],
        };

        let serializer = Serializer::new(main_chunk, obfuscation_context);
        let bytes = serializer.serialze();

        println!("{:?}", bytes);
    }
}
