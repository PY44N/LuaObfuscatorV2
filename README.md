# Lua Obfuscator V2

This project is still a work in progress, so things may not work

[Roadmap](https://pyan.notion.site/014c3553be6b45d1989e1e133ec2c424?v=acc453043e2844728d3db628693c100d)

## How to use
### Required programs
- [Rust](https://www.rust-lang.org/)
- [Nodejs](https://nodejs.org/en)
- Lua 5.1 [windows](https://github.com/rjpcomputing/luaforwindows/releases/), [macos (with homebrew)](https://formulae.brew.sh/formula/lua@5.1#default), Linux (lua5.1 on most package managers)

1) Clone the repo
```
git clone https://github.com/PY44N/LuaObfuscator/
```
2) Enter the directory
```
cd LuaObfuscator
```

3) Install the required nodejs packages
```
cd minifier && npm i && cd ..
```

4) Put the code you wish to obfuscate into a file

4) Run the project using cargo
```
cargo run YOURFILE.lua
```

## Related Repos
[Lua Deserializer](https://github.com/PY44N/LuaDeserializer/) - A library for reading in a serialized Lua binary written for this project
