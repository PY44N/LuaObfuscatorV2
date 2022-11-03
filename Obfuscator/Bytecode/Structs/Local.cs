class Local
{
    private string name;
    private int start;
    private int end;

    public Local()
    {
        name = Deserializer.instance.ReadString();
        start = Deserializer.instance.ReadInt();
        end = Deserializer.instance.ReadInt();
    }
}