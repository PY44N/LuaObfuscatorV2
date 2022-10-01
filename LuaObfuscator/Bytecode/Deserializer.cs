using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LuaObfuscator.Bytecode
{
    internal class Deserializer
    {
        static MemoryStream stream;
        static bool littleEndian = true;
        static int intSize = 4;
        static int sizeTSize = 4;

        static byte[] Read(int bytes)
        {
            byte[] buffer = new byte[bytes];
            stream.Read(buffer, 0, bytes);
            return buffer;
        }

        static int ReadInt8()
        {
            return stream.ReadByte();
        }

        static int ReadInt32()
        {
            return BitConverter.ToInt32(Read(4));
        }

        public static void Deserialize(byte[] compiled)
        {
            stream = new MemoryStream(compiled);

            if (ReadInt32() != 1635077147) // "\27Lua" header
                throw new Exception("Invalid bytecode header");

            if (ReadInt8() != 0x51) // Version number (Lua 5.1)
                throw new Exception("Invalid lua version");

            if (ReadInt8() != 0) // Format version (offical)
                throw new Exception("Invalid format version");

            littleEndian = ReadInt8() == 1; // Endianess little (1) is default
            
            intSize = ReadInt8(); // Int size (should be 4 or 8 bytes)
            if (intSize != 4 && intSize != 8)
                throw new Exception("Invalid int size");

            sizeTSize = ReadInt8(); // SizeT size (should be 4 or 8 bytes)
            if (sizeTSize != 4 && sizeTSize != 8)
                throw new Exception("Invalid SizeT size");

            if (ReadInt8() != 4) // Instruction size (4 bytes)
                throw new Exception("Invalid instruction size");

            if (ReadInt8() != 8) // Lua number size (8 bytes)
                throw new Exception("Invalid lua number size");

            if (ReadInt8() != 0) // Intgral flag 0 = floating point 1 = integral (nope)
                throw new Exception("Invalid integral flag");
        }
    }
}
