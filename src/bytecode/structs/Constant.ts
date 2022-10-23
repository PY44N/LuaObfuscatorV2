import { MemoryStream } from "../../util/MemoryStream";
import { LuaType } from "../enums/LuaType";

export class Constant {
  type: LuaType;
  data: null | boolean | number | string;

  constructor(byteStream: MemoryStream) {
    this.type = byteStream.readInt8();

    switch (this.type) {
      case LuaType.BOOLEAN:
        this.data = byteStream.readInt8() == 1;
        break;
      case LuaType.NUMBER:
        this.data = byteStream.readDouble();
        break;
      case LuaType.STRING:
        this.data = byteStream.readString();
        break;
      default:
        this.data = null;
    }
  }
}
