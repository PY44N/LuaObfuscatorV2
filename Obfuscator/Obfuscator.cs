using System.Diagnostics;

class Obfuscator
{
    public static void obfuscate(string file) {
        if (Directory.Exists("temp")) Directory.Delete("temp", true);
        Directory.CreateDirectory("temp");

        File.Copy(file, "./temp/temp1.lua");

        ProcessStartInfo startInfo = new ProcessStartInfo("luac", "temp1.lua"){
            WorkingDirectory = "./temp",
        };


        Process.Start(startInfo);

        byte[] compiled = File.ReadAllBytes("./temp/luac.out");
        Deserializer deserializer = new Deserializer(compiled);
    } 
}