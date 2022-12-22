use crate::{
    bytecode::structs::chunk::Chunk, obfuscation_settings::ObfuscationSettings,
    util::write_stream::WriteStream,
};

use super::obfuscation_context::ObfuscationContext;

pub struct Serializer {
    main_chunk: Chunk,
}

impl Serializer {
    pub fn new(main_chunk: Chunk) -> Self {
        Self { main_chunk }
    }

    pub fn serialze(
        &self,
        obfuscation_context: &ObfuscationContext,
        settings: &ObfuscationSettings,
    ) -> Vec<u8> {
        let mut write_stream = WriteStream::new();

        self.main_chunk
            .serialize(&mut write_stream, obfuscation_context, settings);

        write_stream.bytes
    }
}
