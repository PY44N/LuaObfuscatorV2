using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LuaObfuscator.Util
{
    internal class Command
    {
        public static void Execute(string command)
        {
            System.Diagnostics.Process.Start("CMD.exe", "/C cd ./temp && " + command).WaitForExit();
        }
    }
}
