pub struct ReadStream {
    pub size_t_size: u8,
    pub int_size: u8,

    bytes: Vec<u8>,
    position: usize,
}

impl ReadStream {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            position: 0,
            size_t_size: 4,
            int_size: 4,
        }
    }

    pub fn read(&mut self, count: usize) -> Vec<u8> {
        //TODO: Endianess
        let ret = self.bytes[self.position..(self.position + count)].to_vec();
        self.position += count;
        ret
    }

    pub fn read_int8(&mut self) -> u8 {
        self.read(1)[0]
    }

    pub fn read_int16(&mut self) -> u16 {
        let bytes = self.read(2);
        bytes[0] as u16 + bytes[1] as u16 * 2u16.pow(8)
    }

    pub fn read_int32(&mut self) -> u32 {
        let bytes = self.read(4);
        bytes[0] as u32
            + bytes[1] as u32 * 2u32.pow(8)
            + bytes[2] as u32 * 2u32.pow(16)
            + bytes[3] as u32 * 2u32.pow(24)
    }

    pub fn read_int64(&mut self) -> u64 {
        self.read_int32() as u64 + self.read_int32() as u64 * 2u64.pow(32)
    }

    pub fn read_int(&mut self) -> u64 {
        if self.int_size == 4 {
            self.read_int32() as u64
        } else {
            self.read_int64()
        }
    }

    pub fn read_size_t(&mut self) -> u64 {
        if self.size_t_size == 4 {
            self.read_int32() as u64
        } else {
            self.read_int64()
        }
    }

    pub fn read_double(&mut self) -> f64 {
        f64::from_le_bytes(self.read(8).try_into().expect("Failed to read double"))
    }

    pub fn read_string_length(&mut self, length: usize) -> String {
        self.read(length).into_iter().map(|v| v as char).collect()
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_size_t() as usize;
        self.read(length).into_iter().map(|v| v as char).collect()
    }
}
