class Chunk
{
    Deserializer deserializer;

    private string sourceName;
    private int lineDefined;
    private int lastLineDefined;
    private int upvalueCount;
    private int parameterCount;
    private int varargFlag;
    private int maxStackSize;

    private Opcode[] instructions;

    public Chunk()
    {
        deserializer = Deserializer.instance;

        sourceName = deserializer.ReadString();
        Console.WriteLine(sourceName);

        lineDefined = deserializer.ReadInt();
        lastLineDefined = deserializer.ReadInt();
        upvalueCount = deserializer.ReadInt8();
        parameterCount = deserializer.ReadInt8();
        varargFlag = deserializer.ReadInt8();
        maxStackSize = deserializer.ReadInt8();

        int instructionCount = deserializer.ReadInt8();
        for (int i = 0; i < instructionCount; i++) {
            //TODO: Instruction size support
            int data = deserializer.ReadInt32();
            instructions.Append(new Opcode(data));
        }
    }
}