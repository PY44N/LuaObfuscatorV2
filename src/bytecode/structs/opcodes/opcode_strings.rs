use crate::bytecode::{enums::opcode_type::OpcodeType, structs::instruction::Instruction};

pub fn get_obfuscated(instruction: &Instruction) -> String {
    //TODO: Write obfuscated instructions
    match instruction.opcode {
        OpcodeType::OpMove => {
            todo!()
        }
        OpcodeType::OpLoadConst => {
            todo!()
        }
        OpcodeType::OpLoadBool => {
            todo!()
        }
        OpcodeType::OpLoadNil => {
            todo!()
        }
        OpcodeType::OpGetUpval => {
            todo!()
        }
        OpcodeType::OpGetGlobal => {
            todo!()
        }
        OpcodeType::OpGetTable => {
            todo!()
        }
        OpcodeType::OpSetGlobal => {
            todo!()
        }
        OpcodeType::OpSetUpval => {
            todo!()
        }
        OpcodeType::OpSetTable => {
            todo!()
        }
        OpcodeType::OpNewTable => {
            todo!()
        }
        OpcodeType::OpSelf => {
            todo!()
        }
        OpcodeType::OpAdd => {
            todo!()
        }
        OpcodeType::OpSub => {
            todo!()
        }
        OpcodeType::OpMul => {
            todo!()
        }
        OpcodeType::OpDiv => {
            todo!()
        }
        OpcodeType::OpMod => {
            todo!()
        }
        OpcodeType::OpPow => {
            todo!()
        }
        OpcodeType::OpUnm => {
            todo!()
        }
        OpcodeType::OpNot => {
            todo!()
        }
        OpcodeType::OpLen => {
            todo!()
        }
        OpcodeType::OpConcat => {
            todo!()
        }
        OpcodeType::OpJmp => {
            todo!()
        }
        OpcodeType::OpEq => {
            todo!()
        }
        OpcodeType::OpLt => {
            todo!()
        }
        OpcodeType::OpLe => {
            todo!()
        }
        OpcodeType::OpTest => {
            todo!()
        }
        OpcodeType::OpTestSet => {
            todo!()
        }
        OpcodeType::OpCall => {
            todo!()
        }
        OpcodeType::OpTailCall => {
            todo!()
        }
        OpcodeType::OpReturn => {
            todo!()
        }
        OpcodeType::OpForLoop => {
            todo!()
        }
        OpcodeType::OpForPrep => {
            todo!()
        }
        OpcodeType::OpTForLoop => {
            todo!()
        }
        OpcodeType::OpSetList => {
            todo!()
        }
        OpcodeType::OpClose => {
            todo!()
        }
        OpcodeType::OpClosure => {
            todo!()
        }
        OpcodeType::OpVarArg => {
            todo!()
        }
    }
}
