import { InstructionType } from "../enums/InstructionType";
import { OpcodeType } from "../enums/OpcodeType";

const instructionMappings: Record<OpcodeType, InstructionType> = {
  [OpcodeType.Move]: InstructionType.ABC,
  [OpcodeType.LoadConst]: InstructionType.ABx,
  [OpcodeType.LoadBool]: InstructionType.ABC,
  [OpcodeType.LoadNil]: InstructionType.ABC,
  [OpcodeType.GetUpval]: InstructionType.ABC,
  [OpcodeType.GetGlobal]: InstructionType.ABx,
  [OpcodeType.GetTable]: InstructionType.ABC,
  [OpcodeType.SetGlobal]: InstructionType.ABx,
  [OpcodeType.SetUpval]: InstructionType.ABC,
  [OpcodeType.SetTable]: InstructionType.ABC,
  [OpcodeType.NewTable]: InstructionType.ABC,
  [OpcodeType.Self]: InstructionType.ABC,
  [OpcodeType.Add]: InstructionType.ABC,
  [OpcodeType.Sub]: InstructionType.ABC,
  [OpcodeType.Mul]: InstructionType.ABC,
  [OpcodeType.Div]: InstructionType.ABC,
  [OpcodeType.Mod]: InstructionType.ABC,
  [OpcodeType.Pow]: InstructionType.ABC,
  [OpcodeType.Unm]: InstructionType.ABC,
  [OpcodeType.Not]: InstructionType.ABC,
  [OpcodeType.Len]: InstructionType.ABC,
  [OpcodeType.Concat]: InstructionType.ABC,
  [OpcodeType.Jmp]: InstructionType.AsBx,
  [OpcodeType.Eq]: InstructionType.ABC,
  [OpcodeType.Lt]: InstructionType.ABC,
  [OpcodeType.Le]: InstructionType.ABC,
  [OpcodeType.Test]: InstructionType.ABC,
  [OpcodeType.TestSet]: InstructionType.ABC,
  [OpcodeType.Call]: InstructionType.ABC,
  [OpcodeType.TailCall]: InstructionType.ABC,
  [OpcodeType.Return]: InstructionType.ABC,
  [OpcodeType.ForLoop]: InstructionType.AsBx,
  [OpcodeType.ForPrep]: InstructionType.AsBx,
  [OpcodeType.TForLoop]: InstructionType.ABC,
  [OpcodeType.SetList]: InstructionType.ABC,
  [OpcodeType.Close]: InstructionType.ABC,
  [OpcodeType.Closure]: InstructionType.ABx,
  [OpcodeType.VarArg]: InstructionType.ABC,
};

export class Opcode {
  data: number;
  opcode: OpcodeType;
  opcodeString: string;
  instructionType: InstructionType;
  dataA: number;
  dataB: number;
  dataC: number | null;

  constructor(data: number) {
    this.data = data;
    this.opcodeString = OpcodeType[this.data & 0x3f];
    // I hate that I have to do this
    this.opcode = Object.values(OpcodeType).indexOf(this.opcodeString);
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

    console.log(
      OpcodeType[this.data & 0x3f],
      this.dataA,
      this.dataB,
      this.dataC
    );
  }
}
