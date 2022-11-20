use super::instruction::Instruction;

pub trait Opcode {
    fn get_instruction(&self) -> &Instruction;
    fn get_obfuscated(&self) -> String;
    fn is_valid(&self) -> bool;
}
