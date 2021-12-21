const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./15.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const grid = rows.map((row) => [...row].map((c) => parseInt(c, 10)));

const riskTo = { "0,0": 0 };
const queue = [[0, 0]];
let position = 0;

while (position < queue.length) {
  // hacky queue, we never delete things
  const l = queue[position];
  const risk = riskTo[`${l[0]},${l[1]}`];

  neighbors(...l).forEach((l1) => {
    const key1 = `${l1[0]},${l1[1]}`;
    if (
      riskTo[key1] === undefined ||
      risk + grid[l1[0]][l1[1]] < riskTo[key1]
    ) {
      riskTo[key1] = risk + grid[l1[0]][l1[1]];
      queue.push(l1);
    }
  });

  position++;
}

console.log(riskTo);
console.log(riskTo[`${grid.length - 1},${grid[0].length - 1}`]);

console.timeEnd("logic");

function neighbors(i, j) {
  return [
    [i - 1, j],
    [i + 1, j],
    [i, j - 1],
    [i, j + 1],
  ].filter(([x, y]) => inside(x, y));
}

function inside(i, j) {
  if (i < 0 || j < 0) {
    return false;
  }

  if (i > grid.length - 1 || j > grid.length - 1) {
    return false;
  }

  return true;
}
