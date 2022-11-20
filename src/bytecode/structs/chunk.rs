use crate::{bytecode::enums::opcode_type::OPCODE_TYPE_MAP, util::memory_stream::MemoryStream};

use super::{constant::Constant, instruction::Instruction, local::Local, opcode::Opcode};

pub struct Chunk {
    pub source_name: String,
    pub line_defined: u64,
    pub last_line_defined: u64,
    pub upvalue_count: u8,
    pub parameter_count: u8,
    pub vararg_flag: u8,
    pub max_stack_size: u8,
    pub instructions: Vec<Box<dyn Opcode>>,
    pub constants: Vec<Constant>,
    pub protos: Vec<Chunk>,
    pub source_lines: Vec<u64>,
    pub locals: Vec<Local>,
    pub upvalues: Vec<String>,
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
            protos: vec![],
            source_lines: vec![],
            locals: vec![],
            upvalues: vec![],
        };

        let instruction_count = memory_stream.read_int();
        for _ in 0..instruction_count {
            //TODO: Instruction size support
            let data = memory_stream.read_int32();
            let opcode_type = OPCODE_TYPE_MAP[(data & 0x3f) as usize];
            // new_self.instructions.push();
        }

        let constant_count = memory_stream.read_int();
        for _ in 0..constant_count {
            new_self.constants.push(Constant::new(memory_stream));
        }

        let proto_count = memory_stream.read_int();
        for _ in 0..proto_count {
            new_self.protos.push(Chunk::new(memory_stream));
        }

        let source_line_count = memory_stream.read_int();
        for _ in 0..source_line_count {
            new_self.source_lines.push(memory_stream.read_int());
        }

        let local_count = memory_stream.read_int();
        for _ in 0..local_count {
            new_self.locals.push(Local::new(memory_stream));
        }

        let upvalue_count = memory_stream.read_int();
        for _ in 0..upvalue_count {
            new_self.upvalues.push(memory_stream.read_string());
        }

        new_self
    }
}
