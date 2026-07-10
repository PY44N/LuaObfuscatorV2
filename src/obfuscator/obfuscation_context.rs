use lua_deserializer::enums::chunk_components::ChunkComponents;

use crate::obfuscator::ir::opcode_type::VMOpcodeType;

use super::vm_generator::ConstantType;

#[derive(Clone)]
pub struct ObfuscationContext {
    pub constant_type_map: [ConstantType; 4],
    pub opcode_map: Vec<VMOpcodeType>,
    pub chunk_component_map: [ChunkComponents; 3],
    pub string_constant_keys: Vec<u8>,
}
