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
    private List<Constant> constants;
    private List<Chunk> protos;
    private List<int> sourceLines;
    private List<Local> locals;
    private List<string> upvalues;

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
            //TODO: Instruction size
            int data = deserializer.ReadInt32();
            instructions.Add(Opcode.Create(data & 0x3f, data));
        }

        constants = new List<Constant>();
        int constantCount = deserializer.ReadInt();
        for (int i = 0; i < constantCount; i++)
        {
            constants.Add(new Constant());
        }

        protos = new List<Chunk>();
        int protoCount = deserializer.ReadInt();
        for (int i = 0; i < protoCount; i++)
        {
            protos.Add(new Chunk());
        }

        sourceLines = new List<int>();
        int sourceLineCount = deserializer.ReadInt();
        for (int i = 0; i < sourceLineCount; i++)
        {
            sourceLines.Add(deserializer.ReadInt());
        }

        locals = new List<Local>();
        int localCount = deserializer.ReadInt();
        for (int i = 0; i < localCount; i++)
        {
            locals.Add(new Local());
        }

        upvalues = new List<string>();
        int upvalueLength = deserializer.ReadInt();
        for (int i = 0; i < upvalueLength; i++)
        {
            upvalues.Add(deserializer.ReadString());
        }
    }
}