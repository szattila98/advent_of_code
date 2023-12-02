const { readFileSync } = require("fs");

const readLines = (path) => {
  return readFileSync(path, { encoding: "utf8", flag: "r" })
    .split(/\r?\n/)
    .filter((line) => !!line);
};

module.exports = { readLines };
