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

   private static readonly Opcode[] opcodes = {
  OpMove,
  OpLoadConst,
  OpLoadBool,
  OpLoadNil,
  OpGetUpval,
  OpGetGlobal,
  OpGetTable,
  OpSetGlobal,
  OpSetUpval,
  OpSetTable,
  OpNewTable,
  OpSelf,
  OpAdd,
  OpSub,
  OpMul,
  OpDiv,
  OpMod,
  OpPow,
  OpUnm,
  OpNot,
  OpLen,
  OpConcat,
  OpJmp,
  OpEq,
  OpLt,
  OpLe,
  OpTest,
  OpTestSet,
  OpCall,
  OpTailCall,
  OpReturn,
  OpForLoop,
  OpForPrep,
  OpTForLoop,
  OpSetList,
  OpClose,
  OpClosure,
  OpVarArg,
   };

    public int opcode;
    public InstructionType instructionType;

    public int dataA;
    public int dataB;
    public int dataC;

    public Opcode()
    {
        //TODO: Instruction size things
        int data = Deserializer.instance.ReadInt32();

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

    public static Opcode Create(int index) {
        // What did I just write?
        object opcodeObject = Activator.CreateInstance(opcodes[index]);
        MethodInfo method = opcodeObject.GetType().GetMethod("get");
        
        object newObject = method.Invoke(opcodeObject, null);
        if (newObject != null) {
            return (Opcode)newObject;
        }
        throw new Exception("Failed to create opcode");
    }

    public Opcode get() {
        return this;
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