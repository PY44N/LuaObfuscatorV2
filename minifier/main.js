const fs = require('fs')
fs.writeFileSync("../temp/temp3.lua", require("./minify").minify(fs.readFileSync("../temp/temp2.lua", "utf8")))