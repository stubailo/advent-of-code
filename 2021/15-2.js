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
    if (riskTo[key1] === undefined || risk + getGrid(...l1) < riskTo[key1]) {
      riskTo[key1] = risk + getGrid(...l1);
      queue.push(l1);
    }
  });

  position++;
}

console.log(riskTo[`${grid.length * 5 - 1},${grid.length * 5 - 1}`]);

// for (let x = 0; x < grid.length * 5; x++) {
//   const row = [];
//   for (let y = 0; y < grid.length * 5; y++) {
//     row.push(getGrid(x, y));
//   }
//   console.log(row.join(''));
// }

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

  if (i > grid.length * 5 - 1 || j > grid.length * 5 - 1) {
    return false;
  }

  return true;
}

function getGrid(x, y) {
  const x1 = x % grid.length;
  const y1 = y % grid.length;

  return (
    ((grid[x1][y1] +
      Math.floor(x / grid.length) +
      Math.floor(y / grid.length) -
      1) %
      9) +
    1
  );
}
