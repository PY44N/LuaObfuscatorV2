use crate::{
    bytecode::enums::{
        constant_instruction_type::{ConstantInstructionType, CONSTANT_INSTRUCTION_MAP},
        instruction_type::{InstructionType, INSTRUCTION_TYPE_MAP},
        opcode_type::{OpcodeType, OPCODE_TYPE_MAP},
    },
    obfuscator::obfuscation_context::ObfuscationContext,
    util::write_stream::WriteStream,
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

    pub fn serialize(
        &self,
        write_stream: &mut WriteStream,
        obfuscation_context: &ObfuscationContext,
    ) {
        let opcode_num = obfuscation_context
            .opcode_map
            .iter()
            .position(|&v| v == self.opcode)
            .unwrap();

        //There is a 100% chance I screwed something up in this mess
        let mut instruction_data: u16 = 0;
        instruction_data |= ((opcode_num as u16) & 0x3f) << 4;
        instruction_data |= (match self.instruction_type {
            InstructionType::ABC => 0b01,
            InstructionType::ABx => 0b10,
            InstructionType::AsBx => 0b11,
        }) << 2;
        instruction_data |= if self.is_constant_b { 1 << 1 } else { 0 };
        instruction_data |= if self.is_constant_c { 1 } else { 0 };

        write_stream.write_int16(instruction_data);
        write_stream.write_int8(self.data_a);

        match self.instruction_type {
            InstructionType::ABC => {
                write_stream.write_int16(self.data_b as u16);
                write_stream.write_int16(self.data_c as u16);
            }
            InstructionType::ABx => write_stream.write_int32(self.data_b as u32),
            InstructionType::AsBx => write_stream.write_int32((self.data_b + 131071) as u32),
        }
    }
}
