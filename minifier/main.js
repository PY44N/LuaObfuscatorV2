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

function renameTableConstruction(tableConstructionExpression, parentPath) {

}

// We need to check for local statement here because we don't want to rename global table values
scan(ast, "LocalStatement", (localStatement) => {
    for (let i in localStatement.init) {
        if (localStatement.init[i].type == "TableConstructorExpression") {
            renameTableConstruction(v, localStatement.variables[i].name)
        }
    }
})

fs.writeFileSync("./out.lua", minifier.minify(ast))