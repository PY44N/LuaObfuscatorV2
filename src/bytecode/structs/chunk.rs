use crate::{
    bytecode::enums::opcode_map::OPCODE_MAP,
    obfuscator::obfuscation_context::{self, ObfuscationContext},
    util::{read_stream::ReadStream, write_stream::WriteStream},
};

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
    pub fn new(memory_stream: &mut ReadStream) -> Self {
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
            let instruction = Instruction::new(data);
            new_self
                .instructions
                .push(OPCODE_MAP[instruction.opcode_number as usize](instruction))
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

    pub fn serialize(
        &self,
        write_stream: &mut WriteStream,
        obfuscation_context: &ObfuscationContext,
    ) {
        //TODO: Serialize debug info
        write_stream.write_int64(self.constants.len() as u64);
        for constant in &self.constants {
            constant.serialize(write_stream, obfuscation_context);
        }
    }
}
