import { Opcode } from "../Opcode";

export class OpGetUpVal extends Opcode {
  getObfuscated(): string {
    throw new Error("Method not implemented.");
  }
}
