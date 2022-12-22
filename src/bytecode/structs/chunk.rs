use crate::{
    obfuscation_settings::ObfuscationSettings,
    obfuscator::obfuscation_context::ObfuscationContext,
    util::{read_stream::ReadStream, write_stream::WriteStream},
};

use super::{constant::Constant, instruction::Instruction, local::Local};

#[derive(Debug)]
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
    pub protos: Vec<Chunk>,
    pub source_lines: Vec<u64>,
    pub locals: Vec<Local>,
    pub upvalues: Vec<String>,
}

impl Chunk {
    pub fn new(memory_stream: &mut ReadStream) -> Self {
        Self {
            source_name: memory_stream.read_string(),
            line_defined: memory_stream.read_int(),
            last_line_defined: memory_stream.read_int(),
            upvalue_count: memory_stream.read_int8(),
            parameter_count: memory_stream.read_int8(),
            vararg_flag: memory_stream.read_int8(),
            max_stack_size: memory_stream.read_int8(),
            instructions: (0..memory_stream.read_int())
                .map(|_| Instruction::new(memory_stream.read_int32()))
                .collect(),
            constants: (0..memory_stream.read_int())
                .map(|_| Constant::new(memory_stream))
                .collect(),
            protos: (0..memory_stream.read_int())
                .map(|_| Chunk::new(memory_stream))
                .collect(),
            source_lines: (0..memory_stream.read_int())
                .map(|_| memory_stream.read_int())
                .collect(),
            locals: (0..memory_stream.read_int())
                .map(|_| Local::new(memory_stream))
                .collect(),
            upvalues: (0..memory_stream.read_int())
                .map(|_| memory_stream.read_string())
                .collect(),
        }
    }

    pub fn serialize(
        &self,
        write_stream: &mut WriteStream,
        obfuscation_context: &ObfuscationContext,
        settings: &ObfuscationSettings,
    ) {
        write_stream.write_string(&self.source_name);
        write_stream.write_int8(self.upvalue_count);
        write_stream.write_int8(self.parameter_count);

        write_stream.write_int64(self.constants.len() as u64);
        for constant in &self.constants {
            constant.serialize(write_stream, obfuscation_context);
        }

        write_stream.write_int64(self.instructions.len() as u64);
        for instruction in &self.instructions {
            instruction.serialize(write_stream, obfuscation_context);
        }

        write_stream.write_int64(self.protos.len() as u64);
        for proto in &self.protos {
            proto.serialize(write_stream, obfuscation_context, settings);
        }

        if settings.include_debug_line_info {
            write_stream.write_int64(self.source_lines.len() as u64);

            for line in &self.source_lines {
                write_stream.write_int64(*line);
            }
        }
    }
}
