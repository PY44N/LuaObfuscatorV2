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

  //From https://gist.github.com/kg/2192799
  readDouble(): number {
    let bytes = this.read(8);
    let littleEndian = true;

    let binary = "";
    for (let i = 0, l = bytes.length; i < l; i++) {
      let bits = bytes[i].toString(2);
      while (bits.length < 8) bits = "0" + bits;

      if (littleEndian) binary = bits + binary;
      else binary += bits;
    }

    let sign = binary.charAt(0) == "1" ? -1 : 1;
    let exponent = parseInt(binary.substring(1, 11), 2) - 1023;
    let significandBase = binary.substring(12, 52);
    let significandBin = "1" + significandBase;
    let i = 0;
    let val = 1;
    let significand = 0;

    if (exponent == -1023) {
      if (significandBase.indexOf("1") == -1) return 0;
      else {
        exponent = -1023;
        significandBin = "0" + significandBase;
      }
    }

    while (i < significandBin.length) {
      significand += val * parseInt(significandBin.charAt(i));
      val = val / 2;
      i++;
    }

    return sign * significand * 2 ** exponent;
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
