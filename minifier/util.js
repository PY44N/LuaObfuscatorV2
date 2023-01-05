function scan(parentChunk, type, callback) {
  for (var i in parentChunk) {
    let currentChunk = parentChunk[i];

    if (typeof currentChunk != "object" || currentChunk == null) {
      continue;
    }

    if (currentChunk.type != null && currentChunk.type == type) {
      callback(currentChunk);
    }

    scan(currentChunk, type, callback);
  }
}

const letters = "abcdefghijklmnopqrstuvwxyz";

function generateVariable(num) {
  let ret = "";

  ret += letters.charAt(num % letters.length);
  num = Math.floor(num / letters.length);

  while (num != 0) {
    ret += letters.charAt(num % letters.length);
    num = Math.floor(num / letters.length);
  }

  return ret;
}

module.exports = {
  scan,
  generateVariable,
};
