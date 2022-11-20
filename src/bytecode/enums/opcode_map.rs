use crate::bytecode::structs::{
    instruction::Instruction, opcode::Opcode, opcodes::op_move::OpMove,
};

pub const OPCODE_MAP: [fn(instruction: Instruction) -> Box<dyn Opcode>; 1] =
    [|inst| Box::new(OpMove::new(inst))];
