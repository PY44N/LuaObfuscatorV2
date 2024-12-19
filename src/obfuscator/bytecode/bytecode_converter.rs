use std::ops::Deref;

use lua::{
    compiler::{DebugLocalInfo, FunctionProto},
    instruction::get_opcode,
};
use lua_deserializer::{
    enums::lua_type::LuaType,
    structs::{chunk::Chunk, constant::Constant, local::Local},
};

pub struct BytecodeConverter;

// TODO: Replace the temporary bytecode converter by standardizing byecode formats
impl BytecodeConverter {
    fn convert_instruction(
        instruction: lua::instruction::Instruction,
    ) -> lua_deserializer::structs::instruction::Instruction {
        lua_deserializer::structs::instruction::Instruction::new(get_opcode(instruction) as u32)
    }

    fn convert_constant(constant: &lua::value::Value) -> Constant {
        match constant {
            lua::value::Value::Nil => Constant {
                lua_type: LuaType::NIL,
            },
            lua::value::Value::Bool(val) => Constant {
                lua_type: LuaType::BOOLEAN(*val),
            },
            lua::value::Value::Number(val) => Constant {
                lua_type: LuaType::NUMBER(*val),
            },
            lua::value::Value::String(val) => Constant {
                lua_type: LuaType::STRING(val.clone()),
            },
            // lua::value::Value::Function => todo!(),
            // lua::value::Value::UserData => todo!(),
            // lua::value::Value::Thread => todo!(),
            // lua::value::Value::Table => todo!(),
            _ => panic!("Unhandled constant type"),
        }
    }

    fn convert_local(info: DebugLocalInfo) -> Local {
        Local {
            name: info.name,
            start: info.spc as u64,
            end: info.epc as u64,
        }
    }

    fn convert(input: FunctionProto) -> Chunk {
        Chunk {
            source_name: input.source,
            line_defined: input.lineinfo.0 as u64,
            last_line_defined: input.lineinfo.1 as u64,
            upvalue_count: input.upval_count,
            parameter_count: input.param_count,
            vararg_flag: input.is_vararg,
            max_stack_size: input.used_registers,
            instructions: input
                .code
                .iter()
                .map(|v| Self::convert_instruction(*v))
                .collect(),
            constants: input
                .constants
                .iter()
                .map(|v| Self::convert_constant(v.deref()))
                .collect(),
            protos: input
                .prototypes
                .iter()
                .map(|v| Self::convert(*v.clone()))
                .collect(),
            source_lines: input.debug_pos.iter().map(|v| *v as u64).collect(),
            locals: input
                .debug_locals
                .iter()
                .map(|v| Self::convert_local(*v.clone()))
                .collect(),
            upvalues: input.debug_upval,
        }
    }
}
