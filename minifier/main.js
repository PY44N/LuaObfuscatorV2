const fs = require("fs");
const luaparse = require("luaparse");
const minifier = require("./minify");
const { scan, generateVariable } = require("./util");

let ast = luaparse.parse(fs.readFileSync("./input.lua", "utf8"));

// fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2));

let numerics = {};

scan(ast, "NumericLiteral", (numeric) => {
  if (numerics[numeric.value] == undefined) {
    numerics[numeric.value] = generateVariable(numerics.length);
  }
  numeric.raw = `numerics.${numerics[numeric.value]}`;
});

let numericString = "local numerics = {";

for (let i in numerics) {
  numericString += `${i == 0 ? "" : ","}${Object.keys(numerics)[i]} = ${
    numerics[i]
  }`;
}

numericString += "}";

fs.writeFileSync(
  "./out.lua",
  minifier.minify(numericString + minifier.minify(ast))
);
