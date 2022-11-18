use crate::{
    bytecode::enums::lua_type::{self, LuaType, LUA_TYPE_MAP},
    util::memory_stream::MemoryStream,
};

pub struct Constant {
    lua_type: LuaType,
    string_data: String,
    number_data: f64,
    bool_data: bool,
}

impl Constant {
    pub fn new(memory_stream: &mut MemoryStream) -> Self {
        let mut new_self = Self {
            lua_type: LUA_TYPE_MAP[memory_stream.read_int8() as usize],
            string_data: String::from(""),
            number_data: 0.0,
            bool_data: false,
        };

        match new_self.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN => new_self.bool_data = memory_stream.read_int8() == 1,
            LuaType::INVALID => {}
            LuaType::NUMBER => new_self.number_data = memory_stream.read_double(),
            LuaType::STRING => new_self.string_data = memory_stream.read_string(),
        }

        new_self
    }
}
