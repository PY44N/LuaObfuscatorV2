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
        assert_eq!(
            self.memory_stream.read_string(4),
            String::from_utf8(vec![27]).unwrap() + "Lua",
            "Invalid file header"
        );

        assert_eq!(
            self.memory_stream.read_int8(),
            0x51,
            "Invalid Lua version, only Lua 5.1 supported"
        );

        assert_eq!(self.memory_stream.read_int8(), 0, "Invalid format version");
    }
}
