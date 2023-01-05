const fs = require("fs");
const luaparse = require("luaparse");
const minifier = require("./minify");

let ast = luaparse.parse(fs.readFileSync("../temp/temp2.lua", "utf8"));

// fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2));

fs.writeFileSync("../temp/temp3.lua", minifier.minify(ast));
