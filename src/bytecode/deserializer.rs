use crate::MemoryStream;

pub struct Deserializer {
    memory_stream: MemoryStream,
}

impl Deserializer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            memory_stream: MemoryStream::new(bytes),
        }
    }

    pub fn deserialize(&mut self) {
        println!("{:?}", self.memory_stream.read_string(4));
    }
}
