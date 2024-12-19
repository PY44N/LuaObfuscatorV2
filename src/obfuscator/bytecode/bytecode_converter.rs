use lua::compiler::{DebugLocalInfo, FunctionProto};
use lua_deserializer::structs::{chunk::Chunk, local::Local};

pub struct BytecodeConverter;

// TODO: Replace the temporary bytecode converter by standardizing byecode formats
impl BytecodeConverter {
    fn convert_local(self, info: DebugLocalInfo) -> Local {
        Local {
            name: info.name,
            start: info.spc as u64,
            end: info.epc as u64,
        }
    }

    fn convert(self, input: FunctionProto) -> Chunk {
        Chunk {
            source_name: input.source,
            line_defined: input.lineinfo.0 as u64,
            last_line_defined: input.lineinfo.1 as u64,
            upvalue_count: input.upval_count,
            parameter_count: input.param_count,
            vararg_flag: input.is_vararg,
            max_stack_size: input.used_registers,
            instructions: (),
            constants: (),
            protos: (),
            source_lines: input.debug_pos.iter().map(|v| *v as u64).collect(),
            locals: input
                .debug_locals
                .iter()
                .map(|v| self.convert_local(**v))
                .collect(),
            upvalues: input.debug_upval,
        }
    }
}
