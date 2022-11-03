class Chunk
{
    private Deserializer deserializer;

    private string sourceName;
    private int lineDefined;
    private int lastLineDefined;
    private int upvalueCount;
    private int parameterCount;
    private int varargFlag;
    private int maxStackSize;

    private List<Opcode> instructions;

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

        instructions = new List<Opcode>();
        int instructionCount = deserializer.ReadInt();
        for (int i = 0; i < instructionCount; i++)
        {
            int data = deserializer.ReadInt32();
            instructions.Add(Opcode.Create(data & 0x3f, data));
        }
    }
}