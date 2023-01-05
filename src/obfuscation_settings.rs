pub struct ObfuscationSettings {
    pub include_debug_line_info: bool,
    pub compress_bytecode: bool,
}

impl ObfuscationSettings {
    pub fn new() -> Self {
        Self {
            include_debug_line_info: false,
            compress_bytecode: true,
        }
    }
}
