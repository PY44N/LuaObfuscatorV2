use crate::bytecode::enums::{
    chunk_components::ChunkComponents, lua_type::LuaType, opcode_type::OpcodeType,
};

#[derive(Clone)]
pub struct ObfuscationContext {
    pub constant_type_map: [LuaType; 4],
    pub opcode_map: Vec<OpcodeType>,
    pub chunk_component_map: [ChunkComponents; 3],
}
