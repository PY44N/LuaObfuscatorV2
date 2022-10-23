import { MemoryStream } from "../../util/MemoryStream";

export class Local {
  name: string;
  start: number;
  end: number;

  constructor(byteStream: MemoryStream) {
    this.name = byteStream.readString();
    this.start = byteStream.readInt();
    this.end = byteStream.readInt();
  }
}
