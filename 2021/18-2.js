const { reverse } = require("dns");
const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./18.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

// l and r should be strings
function sumAndReduceAndMagnitude(stringL, stringR) {
  const l = parse(stringL);
  const r = parse(stringR);
  const first = sum(l, r);
  reduce(first);
  const tree = parseToTree(first);
  return magnitude(tree);
}

let maxMag = 0;
for (let i = 0; i < rows.length - 1; i++) {
  for (let j = i + 1; j < rows.length; j++) {
    const mag1 = sumAndReduceAndMagnitude(rows[i], rows[j]);
    const mag2 = sumAndReduceAndMagnitude(rows[j], rows[i]);

    maxMag = Math.max(maxMag, mag1);
    maxMag = Math.max(maxMag, mag2);
  }
}
console.log(maxMag);

function reduce(first) {
  let hasMore = true;
  while (hasMore) {
    hasMore = explode(first);

    if (!hasMore) {
      hasMore = split(first);
    }

    validate(first);
  }

  return first;
}

function magnitude(t) {
  if (isNumber(t)) {
    return t;
  }

  return magnitude(t[0]) * 3 + magnitude(t[1]) * 2;
}

function parse(str) {
  let first = null;

  let prev = null;
  [...str].forEach((c, i) => {
    const int = parseInt(c, 10);
    let val;

    if (isNumber(int)) {
      val = int;
    } else {
      val = c;
    }

    if (i === 0) {
      prev = { val };
      first = prev;
    } else {
      prev = insertAfter(prev, val);
    }
  });

  return first;
}

function isNumber(c) {
  return !isNaN(c);
}

function sum(l, r) {
  const lastL = getLast(l);
  const lastR = getLast(r);

  stitch(lastL, r);
  const first = insertBefore(l, "[");
  insertBefore(r, ",");
  insertAfter(lastR, "]");

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

      stitch(before, after);
      insertAfter(before, 0);

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

      stitch(before, after);

      let next = insertAfter(before, "[");
      next = insertAfter(next, Math.floor(curr.val / 2));
      next = insertAfter(next, ",");
      next = insertAfter(next, Math.ceil(curr.val / 2));
      next = insertAfter(next, "]");

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

function insertAfter(node, val) {
  const newNode = {
    val,
    prev: node,
    next: node.next,
  };

  if (node.next) {
    node.next.prev = newNode;
  }

  node.next = newNode;

  return newNode;
}

function insertBefore(node, val) {
  const newNode = {
    val,
    next: node,
    prev: node.prev,
  };

  if (node.prev) {
    node.prev.next = newNode;
  }

  node.prev = newNode;

  return newNode;
}

function stitch(l, r) {
  l.next = r;
  r.prev = l;
}

console.timeEnd("logic");
