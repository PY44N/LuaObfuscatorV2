const fs = require("fs");
const luaparse = require("luaparse");
const minifier = require("./minify");
const { scan, generateVariable } = require("./util");

let ast = luaparse.parse(fs.readFileSync("../temp/temp3.lua", "utf8"));
// let ast = luaparse.parse(fs.readFileSync("Input.lua", "utf8"));

// fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2));

let numerics = [];

scan(ast, "NumericLiteral", (numeric) => {
  if (!numerics.includes(numeric.value)) {
    numerics.push(numeric.value);
  }
  numeric.raw = `numericsList.${generateVariable(
    numerics.indexOf(numeric.value)
  )}`;
});

let numericCombinations = []

for (let i in numerics) {
  numericCombinations.push(`${generateVariable(i)} = ${numerics[i]}`)
}

numericCombinations.sort(() => Math.random() - 0.5)

let numericString = "{";

for (let i in numericCombinations) {
  numericString += `${i != 0 ? "," : ""}${numericCombinations[i]}`;
}

numericString += "}";

let strings = [];

scan(ast, "StringLiteral", (string) => {
  const value =
    string.raw.charAt(0) == "["
      ? string.raw.slice(2, -2)
      : string.raw.slice(1, -1);

  if (!strings.includes(value)) {
    strings.push(value);
  }
  string.raw = `stringsList.${generateVariable(strings.indexOf(value))}`;
});

let stringString = "{";

for (let i in strings) {
  stringString += `${i != 0 ? "," : ""}${generateVariable(i)} = [[${
    strings[i]
  }]]`;
}

stringString += "}";

const stringMap = {
  ["numericsList"]: numericString,
  ["stringsList"]: stringString,
};

let funcArgNames = ["numericsList", "stringsList"].sort(() => Math.random() - 0.5)

fs.writeFileSync(
  "../temp/temp4.lua",
  minifier.minify(
    `local main = function(${funcArgNames.toString(",")}) ${minifier.minify(
      ast
    )} end main(${funcArgNames.map((val) => stringMap[val])})`
  )
);
