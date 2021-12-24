const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./18.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const parsed = rows.map(parse);

console.log(parsed.map(toArr));

function parse(str) {
  let first = null;

  let prev = null;
  [...str].forEach((c, i) => {
    const int = parseInt(c, 10);
    const node = {};

    if (isNumber(int)) {
      node.val = int;
    } else {
      node.val = c;
    }

    if (i === 0) {
      first = node;
    } else {
      node.prev = prev;
      prev.next = node;
    }

    prev = node;
  });

  return first;
}

function isNumber(c) {
  return !isNaN(c);
}

function sum(l, r) {
  const first = {
    val: "[",
    next: l,
  };

  const middle = {
    val: ",",
    prev: l,
    next: r,
  };

  const last = {
    val: "]",
    prev: r,
  };

  l.prev = first;
  l.next = middle;
  r.prev = middle;
  r.next = last;

  return first;
}

function toArr(first) {
  const arr = [];
  let next = first;

  while (next) {
    arr.push(next.val);
    next = next.next;
  }

  return arr.join("");
}

console.timeEnd("logic");
