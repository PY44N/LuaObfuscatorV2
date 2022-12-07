use crate::{bytecode::structs::chunk::Chunk, obfuscator::obfuscation_context::ObfuscationContext};

use super::serializer::Serializer;

pub struct VMGenerator {}

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk) {
        let context = ObfuscationContext { debug_info: true };

        let serializer = Serializer::new(main_chunk);
        let bytes = serializer.serialze();

        println!("{:?}", bytes);
    }
}
