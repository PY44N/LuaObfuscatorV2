#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LuaType {
    NIL,
    BOOLEAN,
    INVALID, //Who decided that this was a good idea?
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
