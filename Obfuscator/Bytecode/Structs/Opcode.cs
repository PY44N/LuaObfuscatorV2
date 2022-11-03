using System.Reflection;

abstract class Opcode
{
    private static readonly InstructionType[] instructionMappings = {
        InstructionType.ABC,
        InstructionType.ABx,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABx,
        InstructionType.ABC,
        InstructionType.ABx,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.AsBx,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.AsBx,
        InstructionType.AsBx,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABC,
        InstructionType.ABx,
        InstructionType.ABC,
   };

    private static readonly Func<int, Opcode>[] opcodeConstructors = {
        (int data) => new OpMove(data),
        (int data) => new OpLoadConst(data),
        (int data) => new OpLoadBool(data),
        (int data) => new OpLoadNil(data),
        (int data) => new OpGetUpval(data),
        (int data) => new OpGetGlobal(data),
        (int data) => new OpGetTable(data),
        (int data) => new OpSetGlobal(data),
        (int data) => new OpSetUpval(data),
        (int data) => new OpSetTable(data),
        (int data) => new OpNewTable(data),
        (int data) => new OpSelf(data),
        (int data) => new OpAdd(data),
        (int data) => new OpSub(data),
        (int data) => new OpMul(data),
        (int data) => new OpDiv(data),
        (int data) => new OpMod(data),
        (int data) => new OpPow(data),
        (int data) => new OpUnm(data),
        (int data) => new OpNot(data),
        (int data) => new OpLen(data),
        (int data) => new OpConcat(data),
        (int data) => new OpJmp(data),
        (int data) => new OpEq(data),
        (int data) => new OpLt(data),
        (int data) => new OpLe(data),
        (int data) => new OpTest(data),
        (int data) => new OpTestSet(data),
        (int data) => new OpCall(data),
        (int data) => new OpTailCall(data),
        (int data) => new OpReturn(data),
        (int data) => new OpForLoop(data),
        (int data) => new OpForPrep(data),
        (int data) => new OpTForLoop(data),
        (int data) => new OpSetList(data),
        (int data) => new OpClose(data),
        (int data) => new OpClosure(data),
        (int data) => new OpVarArg(data),
   };

    public int opcode;
    public InstructionType instructionType;

    public int dataA;
    public int dataB;
    public int dataC;

    public Opcode(int data)
    {
        opcode = data & 0x3f;

        instructionType = instructionMappings[opcode];

        dataA = (data >> 6) & 0xff;

        switch (instructionType)
        {
            case InstructionType.ABC:
                //Why Lua?
                dataB = (data >> 23) & 0x1ff;
                dataC = (data >> 14) & 0x1ff;
                break;
            case InstructionType.ABx:
                dataB = (data >> 14) & 0x3ffff;
                dataC = -1;
                break;
            case InstructionType.AsBx:
                dataB = ((data >> 14) & 0x3ffff) - 131071;
                dataC = -1;
                break;
        }
    }

    public static Opcode Create(int code, int data)
    {
        return opcodeConstructors[code].Invoke(data);
    }

    public virtual string getObfuscatated()
    {
        throw new NotImplementedException();
    }
    public virtual void mutate()
    {
        throw new NotImplementedException();
    }
}