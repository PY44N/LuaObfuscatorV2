class Constant
{
    private LuaType type;
    private dynamic? data;

    public Constant()
    {
        type = (LuaType)Deserializer.instance.ReadInt8();

        switch (type)
        {
            case LuaType.BOOLEAN:
                data = Deserializer.instance.ReadInt8() == 1;
                break;
            case LuaType.NUMBER:
                data = Deserializer.instance.ReadDouble();
                break;
            case LuaType.STRING:
                data = Deserializer.instance.ReadString();
                break;
        }
    }
}