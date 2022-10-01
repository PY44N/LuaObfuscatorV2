using LuaObfuscator.Bytecode;
using LuaObfuscator.Util;

class Program
{
    public static void Main(string[] args)
    {
        if (Directory.Exists("temp")) Directory.Delete("temp", true); 
        Directory.CreateDirectory("temp");

        File.Copy("Input.lua", "./temp/temp1.lua");

        Command.Execute("luac temp1.lua");

        byte[] compiled = File.ReadAllBytes("./temp/luac.out");

        Deserializer.Deserialize(compiled);
    }
}

