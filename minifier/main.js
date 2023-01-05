const fs = require("fs");
const luaparse = require("luaparse");
const minifier = require("./minify");
const { scan } = require("./util");

let ast = luaparse.parse(fs.readFileSync("../temp/temp2.lua", "utf8"));

// fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2));

let numerics = [];

scan(ast, "NumericLiteral", (numeric) => {
  if (!numerics.includes(numeric.value)) {
    numerics.push(numeric.value);
  }
  numeric.raw = `numerics[${numerics.indexOf(numeric.value)}]`;
});

let numericString = "local numerics = {";

for (let i in numerics) {
  numericString += `${i != 0 ? "," : ""}${numerics[i]}`;
}

numericString += "}";

fs.writeFileSync(
  "../temp/temp3.lua",
  minifier.minify(numericString + minifier.minify(ast))
);
