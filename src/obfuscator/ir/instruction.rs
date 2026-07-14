use lua_deserializer::{
    enums::{instruction_type::InstructionType, opcode_type::OpcodeType},
    structs::instruction::Instruction,
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

fn get_new_opcode(inst: &Instruction) -> VMOpcodeType {
    let is_constant_b = inst.is_constant_b && inst.data_b > 0xFF;
    let is_constant_c = inst.is_constant_c && inst.data_c > 0xFF;

    match inst.opcode {
        OpcodeType::OpMove => VMOpcodeType::OpMove,
        OpcodeType::OpLoadConst => VMOpcodeType::OpLoadConst,
        OpcodeType::OpLoadBool => VMOpcodeType::OpLoadBool,
        OpcodeType::OpLoadNil => VMOpcodeType::OpLoadNil,
        OpcodeType::OpGetUpval => VMOpcodeType::OpGetUpval,
        OpcodeType::OpGetGlobal => VMOpcodeType::OpGetGlobal,
        OpcodeType::OpGetTable => VMOpcodeType::OpGetTable {
            constant_c: is_constant_c,
        },
        OpcodeType::OpSetGlobal => VMOpcodeType::OpSetGlobal,
        OpcodeType::OpSetUpval => VMOpcodeType::OpSetUpval,
        OpcodeType::OpSetTable => VMOpcodeType::OpSetTable {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpNewTable => VMOpcodeType::OpNewTable,
        OpcodeType::OpSelf => VMOpcodeType::OpSelf {
            constant_c: is_constant_c,
        },
        OpcodeType::OpAdd => VMOpcodeType::OpAdd {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpSub => VMOpcodeType::OpSub {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpMul => VMOpcodeType::OpMul {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpDiv => VMOpcodeType::OpDiv {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpMod => VMOpcodeType::OpMod {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpPow => VMOpcodeType::OpPow {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpUnm => VMOpcodeType::OpUnm,
        OpcodeType::OpNot => VMOpcodeType::OpNot,
        OpcodeType::OpLen => VMOpcodeType::OpLen,
        OpcodeType::OpConcat => VMOpcodeType::OpConcat,
        OpcodeType::OpJmp => VMOpcodeType::OpJmp,
        OpcodeType::OpEq => VMOpcodeType::OpEq {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpLt => VMOpcodeType::OpLt {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpLe => VMOpcodeType::OpLe {
            constant_b: is_constant_b,
            constant_c: is_constant_c,
        },
        OpcodeType::OpTest => VMOpcodeType::OpTest,
        OpcodeType::OpTestSet => VMOpcodeType::OpTestSet,
        OpcodeType::OpCall => VMOpcodeType::OpCall,
        OpcodeType::OpTailCall => VMOpcodeType::OpTailCall,
        OpcodeType::OpReturn => VMOpcodeType::OpReturn,
        OpcodeType::OpForLoop => VMOpcodeType::OpForLoop,
        OpcodeType::OpForPrep => VMOpcodeType::OpForPrep,
        OpcodeType::OpTForLoop => VMOpcodeType::OpTForLoop,
        OpcodeType::OpSetList => VMOpcodeType::OpSetList,
        OpcodeType::OpClose => VMOpcodeType::OpClose,
        OpcodeType::OpClosure => VMOpcodeType::OpClosure,
        OpcodeType::OpVarArg => VMOpcodeType::OpVarArg,
    }
}

impl From<&Instruction> for VMInstruction {
    fn from(value: &Instruction) -> Self {
        Self {
            data: value.data,
            opcode: get_new_opcode(value),
            instruction_type: value.instruction_type.clone(),
            is_constant_b: value.is_constant_b,
            is_constant_c: value.is_constant_c,
            data_a: value.data_a,
            data_b: value.data_b,
            data_c: value.data_c,
        }
    }
}
