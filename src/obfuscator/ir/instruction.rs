use lua_deserializer::{
    enums::instruction_type::InstructionType, structs::instruction::Instruction,
};

use crate::obfuscator::ir::opcode_type::VMOpcodeType;

pub struct VMInstruction {
    pub data: u32,
    pub opcode: VMOpcodeType,
    pub instruction_type: InstructionType,
    pub is_constant_b: bool,
    pub is_constant_c: bool,
    pub data_a: u8,
    pub data_b: i128,
    pub data_c: i64,
}

impl From<Instruction> for VMInstruction {
    fn from(value: Instruction) -> Self {
        Self {
            data: value.data,
            opcode: value.opcode.into(),
            instruction_type: value.instruction_type,
            is_constant_b: value.is_constant_b,
            is_constant_c: value.is_constant_c,
            data_a: value.data_a,
            data_b: value.data_b,
            data_c: value.data_c,
        }
    }
}

impl From<&Instruction> for VMInstruction {
    fn from(value: &Instruction) -> Self {
        Self {
            data: value.data,
            opcode: value.opcode.clone().into(),
            instruction_type: value.instruction_type.clone(),
            is_constant_b: value.is_constant_b,
            is_constant_c: value.is_constant_c,
            data_a: value.data_a,
            data_b: value.data_b,
            data_c: value.data_c,
        }
    }
}

