use crate::util::memory_stream::MemoryStream;

use super::{constant::Constant, instruction::Instruction};

pub struct Chunk {
    pub source_name: String,
    pub line_defined: u64,
    pub last_line_defined: u64,
    pub upvalue_count: u8,
    pub parameter_count: u8,
    pub vararg_flag: u8,
    pub max_stack_size: u8,
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
}

impl Chunk {
    pub fn new(memory_stream: &mut MemoryStream) -> Self {
        let mut new_self = Self {
            source_name: memory_stream.read_string(),
            line_defined: memory_stream.read_int(),
            last_line_defined: memory_stream.read_int(),
            upvalue_count: memory_stream.read_int8(),
            parameter_count: memory_stream.read_int8(),
            vararg_flag: memory_stream.read_int8(),
            max_stack_size: memory_stream.read_int8(),
            instructions: vec![],
            constants: vec![],
        };

        let instruction_count = memory_stream.read_int();
        for _ in 0..instruction_count {
            //TODO: Instruction size support
            let data = memory_stream.read_int32();
            new_self.instructions.push(Instruction::new(data));
        }

        let constant_count = memory_stream.read_int();
        for _ in 0..constant_count {
            new_self.constants.push(Constant::new(memory_stream));
        }

        new_self
    }
}
