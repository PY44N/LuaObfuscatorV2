use std::fmt::Debug;

use super::instruction::Instruction;

pub trait Opcode {
    fn get_instruction(&self) -> &Instruction;
    fn get_obfuscated(&self) -> &str;
    fn is_valid(&self) -> bool;
}

impl Debug for dyn Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.get_instruction())
    }
}
