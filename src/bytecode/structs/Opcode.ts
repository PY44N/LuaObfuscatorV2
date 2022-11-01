import { InstructionType } from "../enums/InstructionType";
import { OpcodeType } from "../enums/OpcodeType";
import { Instruction } from "./Instruction";
import { OpAdd } from "./opcodes/OpAdd";
import { OpCall } from "./opcodes/OpCall";
import { OpClose } from "./opcodes/OpClose";
import { OpClosure } from "./opcodes/OpClosure";
import { OpConcat } from "./opcodes/OpConcat";
import { OpDiv } from "./opcodes/OpDiv";
import { OpEq } from "./opcodes/OpEq";
import { OpForLoop } from "./opcodes/OpForLoop";
import { OpForPrep } from "./opcodes/OpForPrep";
import { OpGetGlobal } from "./opcodes/OpGetGlobal";
import { OpGetTable } from "./opcodes/OpGetTable";
import { OpGetUpval } from "./opcodes/OpGetUpval";
import { OpJmp } from "./opcodes/OpJmp";
import { OpLe } from "./opcodes/OpLe";
import { OpLen } from "./opcodes/OpLen";
import { OpLoadBool } from "./opcodes/OpLoadBool";
import { OpLoadConst } from "./opcodes/OpLoadConst";
import { OpLoadNil } from "./opcodes/OpLoadNil";
import { OpLt } from "./opcodes/OpLt";
import { OpMod } from "./opcodes/OpMod";
import { OpMove } from "./opcodes/OpMove";
import { OpMul } from "./opcodes/OpMul";
import { OpNewTable } from "./opcodes/OpNewTable";
import { OpNot } from "./opcodes/OpNot";
import { OpPow } from "./opcodes/OpPow";
import { OpReturn } from "./opcodes/OpReturn";
import { OpSelf } from "./opcodes/OpSelf";
import { OpSetGlobal } from "./opcodes/OpSetGlobal";
import { OpSetList } from "./opcodes/OpSetList";
import { OpSetTable } from "./opcodes/OpSetTable";
import { OpSetUpval } from "./opcodes/OpSetUpval";
import { OpSub } from "./opcodes/OpSub";
import { OpTailCall } from "./opcodes/OpTailCall";
import { OpTest } from "./opcodes/OpTest";
import { OpTestSet } from "./opcodes/OpTestSet";
import { OpTForLoop } from "./opcodes/OpTForLoop";
import { OpUnm } from "./opcodes/OpUnm";
import { OpVarArg } from "./opcodes/OpVarArg";

const opcodeMappings: Record<OpcodeType, any> = {
  [OpcodeType.Move]: OpMove,
  [OpcodeType.LoadConst]: OpLoadConst,
  [OpcodeType.LoadBool]: OpLoadBool,
  [OpcodeType.LoadNil]: OpLoadNil,
  [OpcodeType.GetUpval]: OpGetUpval,
  [OpcodeType.GetGlobal]: OpGetGlobal,
  [OpcodeType.GetTable]: OpGetTable,
  [OpcodeType.SetGlobal]: OpSetGlobal,
  [OpcodeType.SetUpval]: OpSetUpval,
  [OpcodeType.SetTable]: OpSetTable,
  [OpcodeType.NewTable]: OpNewTable,
  [OpcodeType.Self]: OpSelf,
  [OpcodeType.Add]: OpAdd,
  [OpcodeType.Sub]: OpSub,
  [OpcodeType.Mul]: OpMul,
  [OpcodeType.Div]: OpDiv,
  [OpcodeType.Mod]: OpMod,
  [OpcodeType.Pow]: OpPow,
  [OpcodeType.Unm]: OpUnm,
  [OpcodeType.Not]: OpNot,
  [OpcodeType.Len]: OpLen,
  [OpcodeType.Concat]: OpConcat,
  [OpcodeType.Jmp]: OpJmp,
  [OpcodeType.Eq]: OpEq,
  [OpcodeType.Lt]: OpLt,
  [OpcodeType.Le]: OpLe,
  [OpcodeType.Test]: OpTest,
  [OpcodeType.TestSet]: OpTestSet,
  [OpcodeType.Call]: OpCall,
  [OpcodeType.TailCall]: OpTailCall,
  [OpcodeType.Return]: OpReturn,
  [OpcodeType.ForLoop]: OpForLoop,
  [OpcodeType.ForPrep]: OpForPrep,
  [OpcodeType.TForLoop]: OpTForLoop,
  [OpcodeType.SetList]: OpSetList,
  [OpcodeType.Close]: OpClose,
  [OpcodeType.Closure]: OpClosure,
  [OpcodeType.VarArg]: OpVarArg,
};

export abstract class Opcode {
  abstract instruction: Instruction;

  abstract getObfuscated(): string;
}
