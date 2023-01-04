const fs = require('fs')
const luaparse = require('luaparse');
const minifier = require("./minify")

let ast = luaparse.parse(fs.readFileSync("./input.lua", "utf8"));

fs.writeFileSync("ast.json", JSON.stringify(ast, null, 2))

function scan(parentChunk, type, callback) {
    for (var i in parentChunk)
    {
        let currentChunk = parentChunk[i]
        
        if (typeof currentChunk != "object" || currentChunk == null) {
            continue
        }

        if (currentChunk.type != null && currentChunk.type == type) {
            callback(currentChunk)
        }
        
        scan(currentChunk, type, callback);
    }
}

const letters = "abcdefghijklmnopqrstuvwxyz"

function generateVariable(num) {
  let ret = ""

  ret += letters.charAt(num % letters.length)
  num = Math.floor(num / letters.length)

  while (num != 0) {
    ret += letters.charAt(num % letters.length)
    num = Math.floor(num / letters.length)
  }

  return ret
}

let tableRenameMap = {}

function renameTableConstruction(tableConstructionExpression, parentPath) {
  let varNum = 0;
  for (let v of tableConstructionExpression.fields) {
    const newName = generateVariable(varNum)
    varNum++

    tableRenameMap[parentPath + "." + v.key.name] = parentPath + "." + newName
    
    v.key.name = newName

    if (v.value.type == "TableConstructorExpression") {
      renameTableConstruction(v.value)
    }
  }
}

// We need to check for local statement here because we don't want to rename global table values
scan(ast, "LocalStatement", (localStatement) => {
    for (let i in localStatement.init) {
        if (localStatement.init[i].type == "TableConstructorExpression") {
            renameTableConstruction(localStatement.init[i], localStatement.variables[i].name)
        }
    }
})

scan(ast, "AssignmengtStatement", (assignmentStatement) => {
  
})

fs.writeFileSync("./out.lua", minifier.minify(ast))