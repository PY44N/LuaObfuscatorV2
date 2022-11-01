import { Instruction } from "../Instruction";
import { Opcode } from "../Opcode";

export class OpClose extends Opcode {
  instruction: Instruction;

  constructor(instruction: Instruction) {
    super();

    this.instruction = instruction;
  }

  getObfuscated(): string {
    throw new Error("Method not implemented.");
  }
}
