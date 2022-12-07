use crate::{
    bytecode::{
        enums::{lua_type::LuaType, opcode_type::OpcodeType},
        structs::chunk::Chunk,
    },
    obfuscator::obfuscation_context::ObfuscationContext,
};

use super::serializer::Serializer;

pub struct VMGenerator {}

impl VMGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, main_chunk: Chunk) {
        let obfuscation_context = ObfuscationContext {
            debug_info: true,
            constant_type_map: [
                LuaType::NIL,
                LuaType::BOOLEAN,
                LuaType::NUMBER,
                LuaType::STRING,
            ],
            opcode_map: [
                OpcodeType::OpMove,
                OpcodeType::OpLoadConst,
                OpcodeType::OpLoadBool,
                OpcodeType::OpLoadNil,
                OpcodeType::OpGetUpval,
                OpcodeType::OpGetGlobal,
                OpcodeType::OpGetTable,
                OpcodeType::OpSetGlobal,
                OpcodeType::OpSetUpval,
                OpcodeType::OpSetTable,
                OpcodeType::OpNewTable,
                OpcodeType::OpSelf,
                OpcodeType::OpAdd,
                OpcodeType::OpSub,
                OpcodeType::OpMul,
                OpcodeType::OpDiv,
                OpcodeType::OpMod,
                OpcodeType::OpPow,
                OpcodeType::OpUnm,
                OpcodeType::OpNot,
                OpcodeType::OpLen,
                OpcodeType::OpConcat,
                OpcodeType::OpJmp,
                OpcodeType::OpEq,
                OpcodeType::OpLt,
                OpcodeType::OpLe,
                OpcodeType::OpTest,
                OpcodeType::OpTestSet,
                OpcodeType::OpCall,
                OpcodeType::OpTailCall,
                OpcodeType::OpReturn,
                OpcodeType::OpForLoop,
                OpcodeType::OpForPrep,
                OpcodeType::OpTForLoop,
                OpcodeType::OpSetList,
                OpcodeType::OpClose,
                OpcodeType::OpClosure,
                OpcodeType::OpVarArg,
            ],
        };

        let serializer = Serializer::new(main_chunk, obfuscation_context);
        let bytes = serializer.serialze();

        let bytecode_string: String = bytes
            .into_iter()
            .map(|v| String::from("\\") + &v.to_string())
            .collect();

        println!("{}", bytecode_string);
    }
}
