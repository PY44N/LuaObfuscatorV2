use crate::{
    bytecode::structs::chunk::Chunk,
    util::write_stream::{self, WriteStream},
};

use super::obfuscation_context::ObfuscationContext;

pub struct Serializer {
    main_chunk: Chunk,
    obfuscation_context: ObfuscationContext,
}

impl Serializer {
    pub fn new(main_chunk: Chunk, obfuscation_context: ObfuscationContext) -> Self {
        Self { main_chunk }
    }

    pub fn serialze(&self) -> Vec<u8> {
        let mut write_stream = WriteStream::new();

        self.main_chunk.serialize(&mut write_stream)
    }
}
