#[derive(Clone, Copy)]
pub enum LuaType {
    NIL,
    BOOLEAN,
    INVALID,
    NUMBER,
    STRING,
}

pub static LUA_TYPE_MAP: [LuaType; 5] = [
    LuaType::NIL,
    LuaType::BOOLEAN,
    LuaType::INVALID,
    LuaType::NUMBER,
    LuaType::STRING,
];
