use crate::util::memory_stream::{self, MemoryStream};

pub struct Chunk {
    pub source_name: String,
    pub line_defined: u64,
    pub last_line_defined: u64,
    pub upvalue_count: u8,
    pub parameter_count: u8,
    pub vararg_flag: u8,
    pub max_stack_size: u8,
}

impl Chunk {
    pub fn new(memory_stream: &mut MemoryStream) -> Self {
        Self {
            source_name: memory_stream.read_string(),
            line_defined: memory_stream.read_int(),
            last_line_defined: memory_stream.read_int(),
            upvalue_count: memory_stream.read_int8(),
            parameter_count: memory_stream.read_int8(),
            vararg_flag: memory_stream.read_int8(),
            max_stack_size: memory_stream.read_int8(),
        }
    }
}
