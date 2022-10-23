import { InstructionType } from "../enums/InstructionType";
import { Opcode } from "../enums/Opcode";

const instructionMappings: Record<Opcode, InstructionType> = {
  [Opcode.Move]: InstructionType.ABC,
  [Opcode.LoadConst]: InstructionType.ABx,
  [Opcode.LoadBool]: InstructionType.ABC,
  [Opcode.LoadNil]: InstructionType.ABC,
  [Opcode.GetUpval]: InstructionType.ABC,
  [Opcode.GetGlobal]: InstructionType.ABx,
  [Opcode.GetTable]: InstructionType.ABC,
  [Opcode.SetGlobal]: InstructionType.ABx,
  [Opcode.SetUpval]: InstructionType.ABC,
  [Opcode.SetTable]: InstructionType.ABC,
  [Opcode.NewTable]: InstructionType.ABC,
  [Opcode.Self]: InstructionType.ABC,
  [Opcode.Add]: InstructionType.ABC,
  [Opcode.Sub]: InstructionType.ABC,
  [Opcode.Mul]: InstructionType.ABC,
  [Opcode.Div]: InstructionType.ABC,
  [Opcode.Mod]: InstructionType.ABC,
  [Opcode.Pow]: InstructionType.ABC,
  [Opcode.Unm]: InstructionType.ABC,
  [Opcode.Not]: InstructionType.ABC,
  [Opcode.Len]: InstructionType.ABC,
  [Opcode.Concat]: InstructionType.ABC,
  [Opcode.Jmp]: InstructionType.AsBx,
  [Opcode.Eq]: InstructionType.ABC,
  [Opcode.Lt]: InstructionType.ABC,
  [Opcode.Le]: InstructionType.ABC,
  [Opcode.Test]: InstructionType.ABC,
  [Opcode.TestSet]: InstructionType.ABC,
  [Opcode.Call]: InstructionType.ABC,
  [Opcode.TailCall]: InstructionType.ABC,
  [Opcode.Return]: InstructionType.ABC,
  [Opcode.ForLoop]: InstructionType.AsBx,
  [Opcode.ForPrep]: InstructionType.AsBx,
  [Opcode.TForLoop]: InstructionType.ABC,
  [Opcode.SetList]: InstructionType.ABC,
  [Opcode.Close]: InstructionType.ABC,
  [Opcode.Closure]: InstructionType.ABx,
  [Opcode.VarArg]: InstructionType.ABC,
};

export class Instruction {
  data: number;
  opcode: Opcode;
  opcodeString: string;
  instructionType: InstructionType;
  dataA: number;
  dataB: number;
  dataC: number | null;

  constructor(data: number) {
    this.data = data;
    this.opcodeString = Opcode[this.data & 0x3f];
    // I hate that I have to do this
    this.opcode = Object.values(Opcode).indexOf(this.opcodeString);
    this.instructionType = instructionMappings[this.opcode];

    this.dataA = (this.data >> 6) & 0xff;

    if (this.instructionType == InstructionType.ABC) {
      // Why Lua?
      this.dataB = (this.data >> 23) & 0x1ff;
      this.dataC = (this.data >> 14) & 0x1ff;
    } else if (this.instructionType == InstructionType.ABx) {
      this.dataB = (this.data >> 14) & 0x3ffff;
      this.dataC = -1;
    } else {
      this.dataB = ((this.data >> 14) & 0x3ffff) - 131071;
      this.dataC = -1;
    }

    console.log(Opcode[this.data & 0x3f], this.dataA, this.dataB, this.dataC);
  }
}
