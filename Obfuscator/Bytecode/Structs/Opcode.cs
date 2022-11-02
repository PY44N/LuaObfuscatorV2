abstract class Opcode
{
    private InstructionType[] instructionMappings = {
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

    public virtual string getObfuscatated()
    {
        throw new NotImplementedException();
    }
    public virtual void mutate()
    {
        throw new NotImplementedException();
    }
}