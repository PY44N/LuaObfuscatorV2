import { MemoryStream } from "../util/MemoryStream";

export class Deserializer {
  byteStream: MemoryStream;
  sizeTSize: number = 4;
  intSize: number = 4;

  constructor(bytes: number[]) {
    this.byteStream = new MemoryStream(bytes);
  }

  readSizeT(): number {
    return this.sizeTSize == 4
      ? this.byteStream.readInt32()
      : this.byteStream.readInt64();
  }

  readInt(): number {
    return this.intSize == 4
      ? this.byteStream.readInt32()
      : this.byteStream.readInt64();
  }

  readString(length = this.readSizeT()): string {
    return String.fromCharCode(...this.byteStream.read(length));
  }

  deserialize() {
    if (this.readString(4) != String.fromCharCode(27) + "Lua")
      throw new Error("Invalid file header");

    if (this.byteStream.readInt8() != 0x51)
      throw new Error("Invalid lua version, only Lua 5.1 supported");

    if (this.byteStream.readInt8() != 0)
      throw new Error("Invalid format versions");

    //TODO: Big Endian Support
    //Endianess value (1 = little 0 = big)
    if (this.byteStream.readInt8() != 1) throw new Error("Invalid endianess");

    this.intSize = this.byteStream.readInt8();
    if (this.intSize != 4 && this.intSize != 8)
      throw new Error("Invalid int size");

    this.sizeTSize = this.byteStream.readInt8();
    if (this.sizeTSize != 4 && this.sizeTSize != 8)
      throw new Error("Invalid SizeT size");

    if (this.byteStream.readInt8() != 4)
      throw new Error("Invalid Instruction Size");

    //TODO: Add float (4-bit) number support
    if (this.byteStream.readInt8() != 8)
      throw new Error("Invalid lua number size");

    if (this.byteStream.readInt8() != 0)
      throw new Error("Invalid integral flag");
  }
}
