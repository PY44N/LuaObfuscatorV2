import { MemoryStream } from "../util/MemoryStream";

export class Deserializer {
  byteStream: MemoryStream;

  constructor(bytes: number[]) {
    this.byteStream = new MemoryStream(bytes);
  }
}
