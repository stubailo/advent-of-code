const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./18.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const parsed = rows.map(parse);

function parse(str) {
  let counter = 0;

  return readValue();

  function readValue() {
    if (str[counter] === "[") {
      readChar("[");
      const l = readValue();
      readChar(",");
      const r = readValue();
      readChar("]");
      const node = {l, r};
      l.parent = node;
      r.parent = node;
      return node;
    }

    return readNum();
  }

  function readChar(c) {
    if (str[counter] !== c) {
      throw new Error("expected: " + c);
    }
    counter++;
  }

  function readNum() {
    const val = parseInt(str[counter], 10);
    if (isNaN(val)) {
      throw new Error(
        "expected number, got " + str[counter] + " at " + counter
      );
    }
    counter++;
    return val;
  }
}

let sum = parsed[0];
parsed.forEach((num, i) => {
  if (i === 0) {
    // beginning
    return;
  }

  sum = {l: sum, r: num};
  sum.l.parent = sum;
  sum.r.parent = sum;
  reduce(sum);

  console.log(sum);
});

function reduce(num) {
  while (explode(num, 0)) {}
}

function explode(num, depth) {
  if (depth >= 4 && typeof num.l === 'number' && typeof num.r === 'number') {
    // ???

    return true;
  }

  let exploded = false;
  if (Array.isArray(num.l)) {
    exploded = explode(num.l, depth + 1);
    if (exploded) {
      return true;
    }
  }

  if (Array.isArray(num.r)) {
    exploded = explode(num.r, depth + 1);
    if (exploded) {
      return true;
    }
  }

  return false;
}

console.timeEnd("logic");
