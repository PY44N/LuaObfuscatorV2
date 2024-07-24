use lua_deserializer::enums::{
    chunk_components::ChunkComponents, lua_type::LuaType, opcode_type::OpcodeType,
};

use super::vm_generator::ConstantType;

#[derive(Clone)]
pub struct ObfuscationContext {
    pub constant_type_map: [ConstantType; 4],
    pub opcode_map: Vec<OpcodeType>,
    pub chunk_component_map: [ChunkComponents; 3],
}
