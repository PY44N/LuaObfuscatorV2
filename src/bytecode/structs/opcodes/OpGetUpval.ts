import { Opcode } from "../Opcode";

export class OpGetUpval extends Opcode {
  getObfuscated(): string {
    throw new Error("Method not implemented.");
  }
}
