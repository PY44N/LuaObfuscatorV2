import { Opcode } from "../Opcode";

export class OpSelf extends Opcode {
  getObfuscated(): string {
    throw new Error("Method not implemented.");
  }
}
