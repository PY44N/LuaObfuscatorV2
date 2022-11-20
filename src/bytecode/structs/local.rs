use crate::util::memory_stream::MemoryStream;

pub struct Local {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

impl Local {
    pub fn new(memory_stream: &mut MemoryStream) -> Self {
        Self {
            name: memory_stream.read_string(),
            start: memory_stream.read_int(),
            end: memory_stream.read_int(),
        }
    }
}
