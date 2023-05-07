use crate::{
    enums::lua_type::{LuaType, LUA_TYPE_MAP},
    util::read_stream::ReadStream,
};

#[derive(Debug)]
pub struct Constant {
    pub lua_type: LuaType,
    pub string_data: String,
    pub number_data: f64,
    pub bool_data: bool,
}

impl Constant {
    pub fn new(memory_stream: &mut ReadStream) -> Self {
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
