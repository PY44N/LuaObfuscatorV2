import { MemoryStream } from "../../util/MemoryStream";
import { LuaType } from "../enums/LuaType";
import { Constant } from "./Constant";
import { Instruction } from "./Instruction";

export class Chunk {
  sourceName: string;
  lineDefined: number;
  lastLineDefined: number;
  upvalueCount: number;
  parameterCount: number;
  varargFlag: number;
  maxStackSize: number;
  instructions: Instruction[];
  constants: Constant[];

  constructor(byteStream: MemoryStream) {
    this.sourceName = byteStream.readString();
    console.log(this.sourceName);

    this.lineDefined = byteStream.readInt();
    this.lastLineDefined = byteStream.readInt();
    this.upvalueCount = byteStream.readInt8();
    this.parameterCount = byteStream.readInt8();
    this.varargFlag = byteStream.readInt8();
    this.maxStackSize = byteStream.readInt8();

    this.instructions = [];
    let instructionLength = byteStream.readInt();
    for (let i = 0; i < instructionLength; i++) {
      //TODO: instruction size support
      let data = byteStream.readInt32();
      this.instructions.push(new Instruction(data));
    }

    this.constants = [];
    let constantLength = byteStream.readInt();
    for (let i = 0; i < constantLength; i++) {
      let constant: Constant = new Constant(byteStream.readInt8());

      switch (constant.type) {
        case LuaType.BOOLEAN:
          constant.data = byteStream.readInt8() == 1;
          break;
        case LuaType.NUMBER:
          constant.data = byteStream.readDouble();
          break;
        case LuaType.STRING:
          constant.data = byteStream.readString();
          break;
      }

      this.constants.push(constant);
    }

    let protoLength = byteStream.readInt();
    for (let i = 0; i < protoLength; i++) {}
  }
}
