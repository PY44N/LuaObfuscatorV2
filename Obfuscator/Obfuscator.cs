using System.Diagnostics;

class Obfuscator
{
    public static void obfuscate(string file)
    {
        if (Directory.Exists("temp")) Directory.Delete("temp", true);
        Directory.CreateDirectory("temp");

        File.Copy(file, "./temp/temp1.lua");

        ProcessStartInfo startInfo = new ProcessStartInfo("luac5.1", "temp1.lua")
        {
            WorkingDirectory = "./temp",
        };

        Process.Start(startInfo).WaitForExit();

        byte[] compiled = File.ReadAllBytes("./temp/luac.out");
        Deserializer deserializer = new Deserializer(compiled);
        deserializer.Deserialize();
    }
}