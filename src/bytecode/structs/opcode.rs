pub trait Opcode {
    fn get_obfuscated(&self) -> String;
    fn is_valid(&self) -> bool;
}
