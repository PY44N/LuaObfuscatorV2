import { MemoryStream } from "../../util/MemoryStream";
import { LuaType } from "../enums/LuaType";
import { Constant } from "./Constant";
import { Instruction } from "./Instruction";
import { Local } from "./Local";

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
  protos: Chunk[];
  sourceLines: number[];
  locals: Local[];
  upvalues: string[];

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
      this.constants.push(new Constant(byteStream));
    }

    this.protos = [];
    let protoLength = byteStream.readInt();
    for (let i = 0; i < protoLength; i++) {
      this.protos.push(new Chunk(byteStream));
    }

    this.sourceLines = [];
    let sourceLineLength = byteStream.readInt();
    for (let i = 0; i < sourceLineLength; i++) {
      this.sourceLines.push(byteStream.readInt());
    }

    this.locals = [];
    let localLength = byteStream.readInt();
    for (let i = 0; i < localLength; i++) {
      this.locals.push(new Local(byteStream));
    }

    this.upvalues = [];
    let upvalueLength = byteStream.readInt();
    for (let i = 0; i < upvalueLength; i++) {
      this.upvalues.push(byteStream.readString());
    }
  }
}
