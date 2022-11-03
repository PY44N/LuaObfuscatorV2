class Deserializer
{
    private MemoryStream stream;
    private int intSize = 4;
    private int sizeTSize = 4;

    public static Deserializer instance;

    public Deserializer(byte[] compiled)
    {
        stream = new MemoryStream(compiled);
        instance = this;
    }

    private byte[] Read(int count)
    {
        byte[] bytes = new byte[count];
        stream.Read(bytes, 0, count);
        return bytes;
    }

    public int ReadInt8()
    {
        return Read(1)[0];
    }

    public int ReadInt16()
    {
        return BitConverter.ToUInt16(Read(2));
    }

    public int ReadInt32()
    {
        return (int)BitConverter.ToUInt32(Read(4));
    }

    public int ReadInt64()
    {
        return (int)BitConverter.ToUInt64(Read(8));
    }

    public int ReadSizeT()
    {
        return sizeTSize == 4 ? ReadInt32() : ReadInt64();
    }

    public int ReadInt()
    {
        return intSize == 4 ? ReadInt32() : ReadInt64();
    }

    public string ReadString(int length)
    {
        return string.Join("", Read(length).Select(v => (char)v));
    }

    public string ReadString()
    {
        return ReadString(ReadSizeT());
    }

    public double ReadDouble()
    {
        return BitConverter.ToDouble(Read(8));
    }

    public Chunk Deserialize()
    {
        if (ReadString(4) != (char)27 + "Lua")
            throw new Exception("Invalid file header");

        if (ReadInt8() != 0x51)
            throw new Exception("Invalid lua version");

        if (ReadInt8() != 0)
            throw new Exception("Invalid format versions");

        //TODO: Big Endian Support
        //Endianess value (1 = little 0 = big)
        if (ReadInt8() != 1) throw new Exception("Invalid endianess");

        intSize = ReadInt8();
        if (intSize != 4 && intSize != 8)
            throw new Exception("Invalid int size");

        sizeTSize = ReadInt8();
        if (sizeTSize != 4 && sizeTSize != 8)
            throw new Exception("Invalid SizeT size");

        if (ReadInt8() != 4)
            throw new Exception("Invalid Instruction Size");

        //TODO: Add float (4-bit) number support
        if (ReadInt8() != 8)
            throw new Exception("Invalid lua number size");

        if (ReadInt8() != 0)
            throw new Exception("Invalid integral flag");

        return new Chunk();
    }
}