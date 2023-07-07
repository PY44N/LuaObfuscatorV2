use lua_deserializer::{
    enums::{
        chunk_components::ChunkComponents, instruction_type::InstructionType, lua_type::LuaType,
    },
    structs::{chunk::Chunk, constant::Constant, instruction::Instruction},
    util::write_stream::WriteStream,
};
use magic_crypt::{new_magic_crypt, MagicCrypt128, MagicCryptTrait};

use crate::obfuscation_settings::ObfuscationSettings;

use super::obfuscation_context::ObfuscationContext;

pub struct Serializer {
    write_stream: WriteStream,
    obfuscation_context: ObfuscationContext,
    settings: ObfuscationSettings,
}

impl Serializer {
    pub fn new(obfuscation_context: ObfuscationContext, settings: ObfuscationSettings) -> Self {
        Self {
            write_stream: WriteStream::new(),
            obfuscation_context,
            settings,
        }
    }

    fn serialize_constant(&mut self, constant: &Constant) {
        let type_code = self
            .obfuscation_context
            .constant_type_map
            .iter()
            .position(|&v| v == constant.lua_type)
            .unwrap();

        self.write_stream.write_int8(type_code as u8);

        match constant.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN => {
                self.write_stream
                    .write_int8(if constant.bool_data { 1 } else { 0 })
            }
            LuaType::INVALID => todo!(),
            LuaType::NUMBER => self.write_stream.write_double(constant.number_data),
            LuaType::STRING => self.write_stream.write_string(&constant.string_data),
        }
    }

    fn serialize_instruction(&mut self, instruction: &Instruction) {
        let opcode_num = self
            .obfuscation_context
            .opcode_map
            .iter()
            .position(|&v| v == instruction.opcode)
            .unwrap();

        let mut instruction_data: u16 = 0;
        instruction_data |= ((opcode_num as u16) & 0x3f) << 4;
        instruction_data |= (match instruction.instruction_type {
            InstructionType::ABC => 0b01,
            InstructionType::ABx => 0b10,
            InstructionType::AsBx => 0b11,
        }) << 2;
        instruction_data |= if instruction.is_constant_b { 1 << 1 } else { 0 };
        instruction_data |= if instruction.is_constant_c { 1 } else { 0 };

        self.write_stream.write_int16(instruction_data);
        self.write_stream.write_int8(instruction.data_a);

        match instruction.instruction_type {
            InstructionType::ABC => {
                self.write_stream.write_int16(instruction.data_b as u16);
                self.write_stream.write_int16(instruction.data_c as u16);
            }
            InstructionType::ABx => self.write_stream.write_int32(instruction.data_b as u32),
            InstructionType::AsBx => self
                .write_stream
                .write_int32((instruction.data_b + 131071) as u32),
        }
    }

    fn serialize_chunk(&mut self, chunk: &Chunk) {
        self.write_stream.write_string(&chunk.source_name);
        self.write_stream.write_int8(chunk.upvalue_count);
        self.write_stream.write_int8(chunk.parameter_count);

        for component in self.obfuscation_context.chunk_component_map.clone() {
            match component {
                ChunkComponents::CONSTANTS => {
                    self.write_stream.write_int64(chunk.constants.len() as u64);
                    for constant in &chunk.constants {
                        self.serialize_constant(constant);
                    }
                }
                ChunkComponents::INSTRUCTIONS => {
                    self.write_stream
                        .write_int64(chunk.instructions.len() as u64);
                    for instruction in &chunk.instructions {
                        self.serialize_instruction(instruction);
                    }
                }
                ChunkComponents::PROTOS => {
                    self.write_stream.write_int64(chunk.protos.len() as u64);
                    for proto in &chunk.protos {
                        self.serialize_chunk(proto);
                    }
                }
            }
        }

        if self.settings.include_debug_line_info {
            self.write_stream
                .write_int64(chunk.source_lines.len() as u64);

            for line in &chunk.source_lines {
                self.write_stream.write_int64(*line);
            }
        }
    }

    pub fn serialze(&mut self, main_chunk: Chunk) -> Vec<u8> {
        // self.main_chunk
        //     .serialize(&mut self.write_stream, obfuscation_context, settings);
        self.serialize_chunk(&main_chunk);

        self.write_stream.bytes.clone()
    }
}
