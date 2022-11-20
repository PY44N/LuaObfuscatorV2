use crate::bytecode::structs::chunk::Chunk;

pub struct VMGenerator {}

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk) {}
}
