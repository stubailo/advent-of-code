const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./15.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const grid = rows.map((row) => [...row].map((c) => parseInt(c, 10)));

const cache = {};
function lowestRiskTo(x, y) {
  const key = `${x},${y}`;
  if (cache[key]) {
    return cache[key];
  }

  let result = null;
  if (x === 0 && y === 0) {
    result = 0;
  } else if (x === 0) {
    result = lowestRiskTo(x, y - 1) + grid[x][y];
  } else if (y === 0) {
    result = lowestRiskTo(x - 1, y) + grid[x][y];
  } else {
    result =
      Math.min(lowestRiskTo(x - 1, y), lowestRiskTo(x, y - 1)) + grid[x][y];
  }

  cache[key] = result;

  return result;
}

console.log(lowestRiskTo(grid.length - 1, grid[0].length - 1));

console.timeEnd("logic");
