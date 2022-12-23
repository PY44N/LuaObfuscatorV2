use crate::bytecode::enums::{lua_type::LuaType, opcode_type::OpcodeType};

pub struct ObfuscationContext {
    pub constant_type_map: [LuaType; 4],
    pub opcode_map: Vec<OpcodeType>,
}
