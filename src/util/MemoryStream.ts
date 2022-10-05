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

  //From https://gist.github.com/kg/2192799
  ReadDouble(): number {
    let bytes = this.Read(8);
    let littleEndian = true;

    let binary = "";
    for (let i = 0, l = bytes.length; i < l; i++) {
      let bits = bytes[i].toString(2);
      while (bits.length < 8) bits = "0" + bits;

      if (littleEndian) binary = bits + binary;
      else binary += bits;
    }

    let sign = binary.charAt(0) == "1" ? -1 : 1;
    let exponent = parseInt(binary.substr(1, 11), 2) - 1023;
    let significandBase = binary.substr(12, 52);
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
}
