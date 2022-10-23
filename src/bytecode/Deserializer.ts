import { MemoryStream } from "../util/MemoryStream";
import { Chunk } from "./structs/Chunk";

export class Deserializer {
  byteStream: MemoryStream;

  constructor(bytes: number[]) {
    this.byteStream = new MemoryStream(bytes);
  }

  deserialize(): Chunk {
    if (this.byteStream.readString(4) != String.fromCharCode(27) + "Lua")
      throw new Error("Invalid file header");

    if (this.byteStream.readInt8() != 0x51)
      throw new Error("Invalid lua version, only Lua 5.1 supported");

    if (this.byteStream.readInt8() != 0)
      throw new Error("Invalid format versions");

    //TODO: Big Endian Support
    //Endianess value (1 = little 0 = big)
    if (this.byteStream.readInt8() != 1) throw new Error("Invalid endianess");

    this.byteStream.intSize = this.byteStream.readInt8();
    if (this.byteStream.intSize != 4 && this.byteStream.intSize != 8)
      throw new Error("Invalid int size");

    this.byteStream.sizeTSize = this.byteStream.readInt8();
    if (this.byteStream.sizeTSize != 4 && this.byteStream.sizeTSize != 8)
      throw new Error("Invalid SizeT size");

    if (this.byteStream.readInt8() != 4)
      throw new Error("Invalid Instruction Size");

    //TODO: Add float (4-bit) number support
    if (this.byteStream.readInt8() != 8)
      throw new Error("Invalid lua number size");

    if (this.byteStream.readInt8() != 0)
      throw new Error("Invalid integral flag");

    return new Chunk(this.byteStream);
  }
}
