const fs = require('fs')
const luaparse = require('luaparse');
const { basename } = require('path');
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

function renameTableConstruction(tableConstructionExpression, originalParentPath, renamedParentPath) {
  let varNum = 0;
  for (let v of tableConstructionExpression.fields) {
    const newName = generateVariable(varNum)
    varNum++

    tableRenameMap[originalParentPath + "." + v.key.name] = renamedParentPath + "." + newName
        
    if (v.value.type == "TableConstructorExpression") {
      renameTableConstruction(v.value, originalParentPath + "." + v.key.name, renamedParentPath + "." + newName)
    }
    
    v.key.name = newName
  }
}

// We need to check for local statement here because we don't want to rename global table values
scan(ast, "LocalStatement", (localStatement) => {
    for (let i in localStatement.init) {
        if (localStatement.init[i].type == "TableConstructorExpression") {
            renameTableConstruction(localStatement.init[i], localStatement.variables[i].name, localStatement.variables[i].name)
        }
    }
})

function getFullMemberName(expression) {
  let name = expression.identifier.name;

  if (expression.base.type == "MemberExpression") {
    name = getFullMemberName(expression.base) + "." + name
  } else {
    name = expression.base.name + "." + name
  }

  return name;
}

function renameMemberExpression(expression) {
  const fullName = getFullMemberName(expression)

  if (tableRenameMap[fullName]) {
    const newName = tableRenameMap[fullName];
    expression.identifier.name = newName.slice(newName.lastIndexOf(".") + 1)
  } else {
    let newName = generateVariable(0)
    const parentPath = tableRenameMap[fullName].slice(0, tableRenameMap[fullName].lastIndexOf("."))
//TODO: Stopped here
  }
  
  if (expression.base.type == "MemberExpression") {
    renameMemberExpression(expression.base)
  }

}

scan(ast, "AssignmentStatement", (assignmentStatement) => {
  for (let i in assignmentStatement.variables) {
    if (assignmentStatement.variables[i].type == "MemberExpression") {
      renameMemberExpression(assignmentStatement.variables[i])
    }
  }
})

console.log(tableRenameMap)

fs.writeFileSync("./out.lua", minifier.minify(ast))