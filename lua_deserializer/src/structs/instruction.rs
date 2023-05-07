use crate::enums::{
    constant_instruction_type::{ConstantInstructionType, CONSTANT_INSTRUCTION_MAP},
    instruction_type::{InstructionType, INSTRUCTION_TYPE_MAP},
    opcode_type::{OpcodeType, OPCODE_TYPE_MAP},
};

#[derive(Debug)]
pub struct Instruction {
    pub data: u32,
    pub opcode: OpcodeType,
    pub instruction_type: InstructionType,
    pub is_constant_b: bool,
    pub is_constant_c: bool,
    pub data_a: u8,
    pub data_b: i128,
    pub data_c: i64,
}

impl Instruction {
    pub fn new(data: u32) -> Self {
        let mut new_self = Self {
            data,
            opcode: OPCODE_TYPE_MAP[(data & 0x3f) as usize],
            instruction_type: INSTRUCTION_TYPE_MAP[(data & 0x3f) as usize],
            is_constant_b: false,
            is_constant_c: false,
            data_a: ((data >> 6) & 0xff) as u8,
            data_b: -1,
            data_c: -1,
        };

        let (mode_b, mode_c) = CONSTANT_INSTRUCTION_MAP[new_self.opcode as usize];

        new_self.is_constant_b = mode_b == ConstantInstructionType::OpArgK;
        new_self.is_constant_c = mode_c == ConstantInstructionType::OpArgK;

        new_self.data_b = match new_self.instruction_type {
            InstructionType::ABC => ((data as i128) >> 23) & 0x1ff, //What idiot decided that this should be a thing? I spent way too long with the registers flipped because some imbecile twenty years ago decided that C should come before B
            InstructionType::ABx => ((data as i128) >> 14) & 0x3ffff,
            InstructionType::AsBx => (((data as i128) >> 14) & 0x3ffff) - 131071,
        };

        if new_self.instruction_type == InstructionType::ABC {
            new_self.data_c = ((data >> 14) & 0x1ff) as i64;
        }

        new_self
    }
}
