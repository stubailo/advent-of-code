const { reverse } = require("dns");
const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./18.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const parsed = rows.map(parse);

let first = null;
parsed.forEach((p, i) => {
  if (i === 0) {
    first = p;
  } else {
    first = sum(first, p);
  }
});

let hasMore = true;

while (hasMore) {
  hasMore = explode(first);

  if (hasMore) {
    console.log("exploded!");
  } else {
    hasMore = split(first);
    if (hasMore) {
      console.log("split!");
    }
  }

  validate(first);
}

const tree = parseToTree(first);

function magnitude(t) {
  if (isNumber(t)) {
    return t;
  }

  return magnitude(t[0]) * 3 + magnitude(t[1]) * 2;
}

console.log(magnitude(tree));

function parse(str) {
  let first = null;

  let prev = null;
  [...str].forEach((c, i) => {
    const int = parseInt(c, 10);
    const node = {};

    if (isNumber(int)) {
      if (isNumber(prev.val)) {
        prev.val = prev.val * 10 + int;
        return;
      }

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
  const lastL = getLast(l);
  const lastR = getLast(r);

  const first = {
    val: "[",
    next: l,
  };

  const middle = {
    val: ",",
    prev: lastL,
    next: r,
  };

  const last = {
    val: "]",
    prev: lastR,
  };

  l.prev = first;
  lastL.next = middle;
  r.prev = middle;
  lastR.next = last;

  return first;
}

function getLast(first) {
  while (first.next) {
    first = first.next;
  }
  return first;
}

function toStrBackwards(first) {
  const arr = [];
  let prev = getLast(first);

  while (prev) {
    arr.push(prev.val);
    prev = prev.prev;
  }

  arr.reverse();

  return arr.join("");
}

function toStr(first) {
  const arr = [];
  let next = first;

  while (next) {
    arr.push(next.val);
    next = next.next;
  }

  return arr.join("");
}

function explode(first) {
  let nesting = 0;
  let curr = first;
  while (curr) {
    if (
      nesting > 4 &&
      isNumber(curr.val) &&
      curr.next && // ,
      curr.next.next &&
      isNumber(curr.next.next.val)
    ) {
      let movingLeft = curr.prev;
      while (movingLeft && !isNumber(movingLeft.val)) {
        movingLeft = movingLeft.prev;
      }
      if (movingLeft) {
        movingLeft.val += curr.val;
      }

      let movingRight = curr.next.next.next;
      while (movingRight && !isNumber(movingRight.val)) {
        movingRight = movingRight.next;
      }
      if (movingRight) {
        movingRight.val += curr.next.next.val;
      }

      const before = curr.prev.prev;
      const after = curr.next.next.next.next;
      const zero = { val: 0, prev: before, next: after };
      before.next = zero;
      after.prev = zero;

      return true;
    } else if (curr.val === "[") {
      nesting++;
    } else if (curr.val === "]") {
      nesting--;
    }

    curr = curr.next;
  }

  return false;
}

function split(first) {
  let curr = first;
  while (curr) {
    if (isNumber(curr.val) && curr.val >= 10) {
      const before = curr.prev;
      const after = curr.next;
      const pair = parse(
        `[${Math.floor(curr.val / 2)},${Math.ceil(curr.val / 2)}]`
      );

      before.next = pair;
      pair.prev = before;

      after.prev = getLast(pair);
      getLast(pair).next = after;

      return true;
    }

    curr = curr.next;
  }

  return false;
}

function parseToTree(first) {
  let curr = first;

  function parseAny() {
    if (isNumber(curr.val)) {
      const result = curr.val;
      curr = curr.next;
      return result;
    }

    return parsePair();
  }

  function parsePair() {
    readChar("[");
    const l = parseAny();
    readChar(",");
    const r = parseAny();
    readChar("]");

    return [l, r];
  }

  function readChar(char) {
    if (curr.val !== char) {
      throw new Error("expected char: " + char);
    }
    curr = curr.next;
  }

  return parseAny();
}

function validate(first) {
  if (/\[0\]/.test(toStr(first))) {
    throw new Error("oh no!!");
  }
}

console.timeEnd("logic");
