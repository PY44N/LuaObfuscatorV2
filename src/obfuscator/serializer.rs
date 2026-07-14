use lua_deserializer::{
    enums::{
        chunk_components::ChunkComponents, instruction_type::InstructionType, lua_type::LuaType,
    },
    structs::{chunk::Chunk, constant::Constant},
    util::write_stream::WriteStream,
};

use crate::{
    obfuscation_settings::ObfuscationSettings,
    obfuscator::{ir::instruction::VMInstruction, utils::index_of, vm_generator::ConstantType},
};

use super::obfuscation_context::ObfuscationContext;

fn xor_position_dependant_key(bytes: &[u8], key: u8) -> Vec<u8> {
    let mut new_bytes = Vec::new();
    for i in 0..bytes.len() {
        new_bytes.push(bytes[i] ^ ((key as usize + i) % 255) as u8);
    }

    new_bytes
}

pub struct Serializer {
    write_stream: WriteStream,
    obfuscation_context: ObfuscationContext,
    settings: ObfuscationSettings,
    string_const_serialized_count: usize,
}

impl Serializer {
    pub fn new(obfuscation_context: ObfuscationContext, settings: ObfuscationSettings) -> Self {
        Self {
            write_stream: WriteStream::new(),
            obfuscation_context,
            settings,
            string_const_serialized_count: 0,
        }
    }

    fn serialize_constant(&mut self, constant: &Constant) {
        self.write_stream.write_int8(match constant.lua_type {
            LuaType::NIL => index_of(
                &self.obfuscation_context.constant_type_map,
                ConstantType::NIL,
            )
            .try_into()
            .unwrap(),
            LuaType::BOOLEAN(_) => index_of(
                &self.obfuscation_context.constant_type_map,
                ConstantType::BOOLEAN,
            )
            .try_into()
            .unwrap(),
            LuaType::INVALID => unreachable!(),
            LuaType::NUMBER(_) => index_of(
                &self.obfuscation_context.constant_type_map,
                ConstantType::NUMBER,
            )
            .try_into()
            .unwrap(),
            LuaType::STRING(_) => index_of(
                &self.obfuscation_context.constant_type_map,
                ConstantType::STRING,
            )
            .try_into()
            .unwrap(),
        });

        match &constant.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN(data) => self.write_stream.write_int8(if *data { 1 } else { 0 }),
            LuaType::INVALID => unreachable!(),
            LuaType::NUMBER(data) => self.write_stream.write_double(*data),
            LuaType::STRING(data) => {
                let mut encrypted_data = xor_position_dependant_key(
                    data.as_bytes(),
                    self.obfuscation_context.string_constant_keys[self
                        .string_const_serialized_count
                        % self.obfuscation_context.string_constant_keys.len()],
                );
                // self.write_stream.write_string(&data);

                // TODO: This shouldn't have to be mutable
                self.write_stream.write_size_t(encrypted_data.len() as u64);
                self.write_stream.write(&mut encrypted_data);
                self.string_const_serialized_count += 1;
            }
        }
    }

    fn serialize_instruction(&mut self, instruction: &VMInstruction) {
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
            InstructionType::AsBx => self.write_stream.write_int32((instruction.data_b) as u32),
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
                    let vm_instructions: Vec<VMInstruction> =
                        chunk.instructions.iter().map(|i| i.into()).collect();
                    self.write_stream
                        .write_int64(chunk.instructions.len() as u64);
                    for instruction in &vm_instructions {
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
