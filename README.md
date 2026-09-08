# Lua Obfuscator V2

This project is still a work in progress, so things may not work

Please download the latest release if you are trying to run the code

## Example
Input
```lua
print("Hello World")
```

[Output](https://raw.githubusercontent.com/PY44N/LuaObfuscatorV2/master/Example.lua)

## How to use
### Required programs
- [Nodejs](https://nodejs.org/en)
- Lua 5.1 [windows](https://github.com/rjpcomputing/luaforwindows/releases/), [macos (with homebrew)](https://formulae.brew.sh/formula/lua@5.1#default), Linux (lua5.1 on most package managers)

### Running the latest release (Windows & Linux)
1) Download the zip for your OS from the [Latest Release](https://github.com/PY44N/LuaObfuscatorV2/releases/) (`LuaObfuscator-windows.zip` or `LuaObfuscator-linux.zip`)

Note: There is currently no prebuilt macOS binary. Mac users should build from source (see below)

2) Open the terminal in the unzipped directory

3) Install the required nodejs packages
```
cd minifier && npm i && cd ..
```

4) Put the code you wish to obfuscate into a file

5) Run the executable
```
# Windows
./lua_obfuscator.exe --file ./YOURFILE.lua --run

# Linux
./lua_obfuscator --file ./YOURFILE.lua --run
```

### CLI options
| Flag | Default | Description |
| --- | --- | --- |
| `-f, --file <FILE>` | required | Lua file to obfuscate |
| `-r, --run` | off | Run the obfuscated output after building it |
| `--no-encrypt-strings` | off (strings are encrypted) | Disable the extra layer of string encryption |
| `--no-compress-bytecode` | off (bytecode is compressed) | Disable bytecode compression |
| `--no-constant-mutations` | off (constants are mutated) | Disable constant mutation |
| `--include-debug-line-info` | off | Include debug line info so runtime errors report a source line |

### Building from source (may not work)
1) Download [Rust](https://www.rust-lang.org/)

2) Clone the repo
```
git clone https://github.com/PY44N/LuaObfuscatorV2/
```

3) Enter the directory
```
cd LuaObfuscatorV2
```

4) Install the required nodejs packages
```
cd minifier && npm i && cd ..
```

5) Put the code you wish to obfuscate into a file

6) Run the project using cargo
```
cargo run -- --file YOURFILE.lua
```

## Related Repos
[Lua Deserializer](https://github.com/PY44N/LuaDeserializer/) - A library for reading in a serialized Lua binary written for this project

[luamin](https://github.com/mathiasbynens/luamin) (by Mathias Bynens) - A Lua minifier written in Javascript that is being used as a temporary solution until the [minification rework](https://pyan.notion.site/014c3553be6b45d1989e1e133ec2c424?v=acc453043e2844728d3db628693c100d&p=597187d43f014c02b3f61fb70aaed968&pm=s)

[FiOne](https://github.com/Rerumu/FiOne/blob/master/source.lua) (by Rerumu) - Lua bytecode interpreter
