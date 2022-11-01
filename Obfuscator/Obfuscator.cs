using System.Diagnostics;

class Obfuscator
{
    public static void obfuscate(string file) {
        if (Directory.Exists("temp")) Directory.Delete("temp");
        Directory.CreateDirectory("temp");

        File.Copy(file, "./temp/temp1.lua");

        ProcessStartInfo startInfo = new ProcessStartInfo();

        startInfo.WorkingDirectory = "./temp";
        startInfo.FileName = "luac";

        Process.Start("luac", "temp1.lua");
    } 
}