import { InstructionType } from "../enums/InstructionType";
import { OpcodeType } from "../enums/OpcodeType";
import { Instruction } from "./Instruction";
import { OpMove } from "./opcodes/OpMove";

// const opcodeMappings: Record<OpcodeType, any> = {
//   [OpcodeType.Move]: OpMove,
// };

export abstract class Opcode {
  instruction: Instruction;

  constructor(instruction: Instruction) {
    this.instruction = instruction;
  }

  abstract getObfuscated(): string;
}
