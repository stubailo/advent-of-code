const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./12.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

// create a mapping from a node to all the nodes that can be reached from it
const map = {};
rows.forEach((row) => {
  const [a, b] = row.split('-');
  if (!map[a]) {
    map[a] = [];
  }
  map[a].push(b);

  if (!map[b]) {
    map[b] = [];
  }
  map[b].push(a);
});

const pathsToEnd = [];
const stack = [['start']];
function expand(path) {
  const last = path[path.length - 1];
  map[last].forEach((next) => {
    if (next === 'end') {
      pathsToEnd.push([...path, next]);
    } else if (next === next.toUpperCase() || path.indexOf(next) === -1) {
      stack.push([...path, next]);
    } else if (next !== 'start' && !hasTwoLowercase(path)) {
      stack.push([...path, next]);
    }
  });
}

while(stack.length) {
  expand(stack.pop());
}

console.log(pathsToEnd.length);

console.timeEnd("logic");

function hasTwoLowercase(path) {
  const count = {};
  for (node of path) {
    if (node.toLowerCase() === node) {
      if (count[node]) {
        return true;
      }
      count[node] = true;
    }
  }
  return false;
}