import { MemoryStream } from "../util/MemoryStream";

export class Deserializer {
  byteStream: MemoryStream;
  sizeTSize: number = 4;
  intSize: number = 4;

  constructor(bytes: number[]) {
    this.byteStream = new MemoryStream(bytes);

    console.log(this.byteStream.readDouble());
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

  deserialize() {}
}
