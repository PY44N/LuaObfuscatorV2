pub struct WriteStream {
    bytes: Vec<u8>,
}

impl WriteStream {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn write(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
    }

    pub fn write_int8(&mut self, num: u8) {
        self.write(&mut vec![num]);
    }

    pub fn write_int16(&mut self, num: u16) {
        self.write(&mut vec![
            (num & 0xff).try_into().unwrap(),
            ((num >> 8) & 0xff).try_into().unwrap(),
        ]);
    }

    pub fn write_int32(&mut self, num: u32) {
        self.write(&mut vec![
            (num & 0xff).try_into().unwrap(),
            ((num >> 8) & 0xff).try_into().unwrap(),
            ((num >> 16) & 0xff).try_into().unwrap(),
            ((num >> 24) & 0xff).try_into().unwrap(),
        ]);
    }

    pub fn write_int64(&mut self, num: u64) {
        self.write_int32((num & 0xffffffff).try_into().unwrap());
        self.write_int32(((num >> 32) & 0xffffffff).try_into().unwrap());
    }

    pub fn write_double(&mut self, num: f64) {
        self.write(&mut f64::to_le_bytes(num).to_vec());
    }

    pub fn write_string(&mut self, str: &String) {
        self.write(&mut str.as_bytes().to_vec());
    }
}
