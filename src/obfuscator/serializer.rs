use crate::{
    bytecode::structs::chunk::Chunk,
    util::write_stream::{self, WriteStream},
};

pub struct Serializer {
    main_chunk: Chunk,
}

impl Serializer {
    pub fn new(main_chunk: Chunk) -> Self {
        Self { main_chunk }
    }

    pub fn serialze(&self) -> Vec<u8> {
        let mut write_stream = WriteStream::new();

        self.main_chunk.serialize(&mut write_stream)
    }
}
