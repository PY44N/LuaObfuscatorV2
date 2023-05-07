use crate::{structs::chunk::Chunk, util::read_stream::ReadStream};

pub struct Deserializer {
    memory_stream: ReadStream,
}

impl Deserializer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            memory_stream: ReadStream::new(bytes),
        }
    }

    pub fn deserialize(&mut self) -> Chunk {
        assert_eq!(
            self.memory_stream.read_string_length(4),
            String::from_utf8(vec![27]).unwrap() + "Lua",
            "Invalid file header"
        );

        assert_eq!(
            self.memory_stream.read_int8(),
            0x51,
            "Invalid Lua version, only Lua 5.1 supported"
        );

        assert_eq!(self.memory_stream.read_int8(), 0, "Invalid format version");

        //TODO: Alternate endianess
        // Endianess value (1 = little 0 = big)
        assert_eq!(self.memory_stream.read_int8(), 1, "Invalid endianess value");

        self.memory_stream.int_size = self.memory_stream.read_int8();
        assert!(
            self.memory_stream.int_size == 4 || self.memory_stream.int_size == 8,
            "Invalid int size"
        );

        self.memory_stream.size_t_size = self.memory_stream.read_int8();
        assert!(
            self.memory_stream.size_t_size == 4 || self.memory_stream.size_t_size == 8,
            "Invalid size t size"
        );

        assert_eq!(
            self.memory_stream.read_int8(),
            4,
            "Invalid instruction size"
        );

        //TODO: Float (4-bit) number support
        assert_eq!(self.memory_stream.read_int8(), 8, "Invalid lua number size");

        assert_eq!(self.memory_stream.read_int8(), 0, "Invalid integral flag");

        Chunk::new(&mut self.memory_stream)
    }
}
