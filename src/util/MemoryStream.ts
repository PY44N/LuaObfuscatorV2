export class MemoryStream {
  bytes: number[];
  position: number;

  constructor(bytes: number[]) {
    this.bytes = bytes;
    this.position = 0;
  }

  Read(count: number): number[] {
    //TODO: Endianess Things
    return this.bytes.slice(this.position, (this.position += count));
  }

  ReadInt8(): number {
    return this.Read(1)[0];
  }

  ReadInt16(): number {
    let [a, b] = this.Read(2);
    return a + b * 2 ** 8;
  }

  ReadInt32(): number {
    let [a, b, c, d] = this.Read(4);
    return a + b * 2 ** 8 + c * 2 ** 16 + d * 2 ** 24;
  }

  ReadInt64(): number {
    return this.ReadInt32() + this.ReadInt32() * 2 ** 32;
  }
}
