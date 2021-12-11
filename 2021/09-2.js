const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./09.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const nums = rows.map((row) => row.split("").map((char) => parseInt(char, 10)));

const lowPoints = [];
for (let i = 0; i < rows.length; i++) {
  for (let j = 0; j < rows[0].length; j++) {
    const item = get(i, j);
    if (
      item < get(i - 1, j) &&
      item < get(i, j - 1) &&
      item < get(i + 1, j) &&
      item < get(i, j + 1)
    ) {
      lowPoints.push([i, j]);
    }
  }
}

const basinSizes = [];

// gen basin from each one
lowPoints.forEach((lp) => {
  const pointsInBasin = {};

  let pointsToExpand = [lp];
  let nextPointsToExpand = [];
  while (pointsToExpand.length) {
    pointsToExpand.forEach(([i, j]) => {
      if (
        i < 0 ||
        j < 0 ||
        i > nums.length - 1 ||
        j > nums.length - 1 ||
        nums[i][j] === 9
      ) {
        return;
      }

      const str = `${i},${j}`;
      if (pointsInBasin[str]) {
        return;
      }

      nextPointsToExpand.push([i - 1, j], [i + 1, j], [i, j - 1], [i, j + 1]);

      pointsInBasin[str] = true;
    });

    pointsToExpand = nextPointsToExpand;
    nextPointsToExpand = [];
  }

  basinSizes.push(Object.keys(pointsInBasin).length);
});

basinSizes.sort((a, b) => b - a);
console.log(basinSizes, basinSizes[0] * basinSizes[1] * basinSizes[2]);

console.timeEnd("logic");

function get(i, j) {
  if (i < 0 || j < 0) {
    return Infinity;
  }

  if (i > nums.length - 1 || j > nums.length - 1) {
    return Infinity;
  }

  return nums[i][j];
}
