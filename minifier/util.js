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

module.exports = {
  scan,
};
