#[derive(Clone)]
pub struct ObfuscationSettings {
    pub include_debug_line_info: bool,
    pub compress_bytecode: bool,
    pub encrypt_strings: bool,
    pub constant_mutations: bool,
}
