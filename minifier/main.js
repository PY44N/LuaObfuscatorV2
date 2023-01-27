const fs = require("fs");
const luaparse = require("luaparse");
const minifier = require("./minify");
const { scan, generateVariable } = require("./util");

// let ast = luaparse.parse(fs.readFileSync("../temp/temp2.lua", "utf8"));
let ast = luaparse.parse(fs.readFileSync("Input.lua", "utf8"));

fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2));

const arithmeticOperators = ["+", "-", "*", "/"];

const equalityOperators = ["==", "~="];

scan(ast, "BinaryExpression", (binExpr) => {
  if (arithmeticOperators.includes(binExpr.operator)) {
    function handleExpression(expr) {
      if (expr.type == "NumericLiteral") {
        const operator = Math.round(Math.random()) == 0 ? "+" : "-";
        const rhs = Math.floor(Math.random() * 100) + 1;
        const lhs = operator == "+" ? expr.value - rhs : expr.value + rhs;
        expr.raw = `${lhs} ${operator} ${rhs}`;
      }
    }

    handleExpression(binExpr.left);
    handleExpression(binExpr.right);
  }

  if (equalityOperators.includes(binExpr.operator)) {
    let old = binExpr;
    old.operator = binExpr.operator == "==" ? "~=" : "==";
    binExpr = {
      type: "UnaryExpression",
      operator: "not",
      argument: {
        old,
      },
    };
  }
});

// We don't talk about this
ast = luaparse.parse(minifier.minify(ast));

let numerics = [];

scan(ast, "NumericLiteral", (numeric) => {
  if (!numerics.includes(numeric.value)) {
    numerics.push(numeric.value);
  }
  numeric.raw = `numericsList.${generateVariable(
    numerics.indexOf(numeric.value)
  )}`;
});

let numericString = "{";

for (let i in numerics) {
  numericString += `${i != 0 ? "," : ""}${generateVariable(i)} = ${
    numerics[i]
  }`;
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

let funcArgNames = ["numericsList", "stringsList"]
  .map((value) => ({ value, sort: Math.random() }))
  .sort((a, b) => a.sort - b.sort)
  .map(({ value }) => value);

fs.writeFileSync(
  "Out.lua",
  minifier.minify(
    `local main = function(${funcArgNames.toString(",")}) ${minifier.minify(
      ast
    )} end main(${funcArgNames.map((val) => stringMap[val])})`
  )
);
