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
    }
}