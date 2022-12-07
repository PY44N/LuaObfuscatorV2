use crate::bytecode::enums::lua_type::LuaType;

pub struct ObfuscationContext {
    pub debug_info: bool,
    pub constant_type_map: [LuaType; 4],
}
