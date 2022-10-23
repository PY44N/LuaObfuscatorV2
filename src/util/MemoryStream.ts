export class MemoryStream {
  bytes: number[];
  position: number;
  sizeTSize: number;
  intSize: number;

  constructor(bytes: number[]) {
    this.bytes = bytes;
    this.position = 0;
    this.sizeTSize = 4;
    this.intSize = 4;
  }

  read(count: number): number[] {
    //TODO: Endianess Things
    return this.bytes.slice(this.position, (this.position += count));
  }

  readInt8(): number {
    return this.read(1)[0];
  }

  readInt16(): number {
    let [a, b] = this.read(2);
    return a + b * 2 ** 8;
  }

  readInt32(): number {
    let [a, b, c, d] = this.read(4);
    return a + b * 2 ** 8 + c * 2 ** 16 + d * 2 ** 24;
  }

  readInt64(): number {
    return this.readInt32() + this.readInt32() * 2 ** 32;
  }

  readDouble(): number {
    return Buffer.from(this.read(8)).readDoubleLE();
  }

  readSizeT(): number {
    return this.sizeTSize == 4 ? this.readInt32() : this.readInt64();
  }

  readInt(): number {
    return this.intSize == 4 ? this.readInt32() : this.readInt64();
  }

  readString(length = this.readSizeT()): string {
    return String.fromCharCode(...this.read(length));
  }
}
