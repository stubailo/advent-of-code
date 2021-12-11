const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./05.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();


const parsed = rows.map((row) =>
  row.split(" -> ").map((pair) => pair.split(",").map((v) => parseInt(v, 10)))
);
const points = {};
console.time("logic");

parsed.forEach(([start, end]) => {
  if (start[0] === end[0]) {
    const [min, max] =
      start[1] < end[1] ? [start[1], end[1]] : [end[1], start[1]];
    for (let i = min; i <= max; i++) {
      const pt = `${start[0]}, ${i}`;
      if (points[pt]) {
        points[pt] += 1;
      } else {
        points[pt] = 1;
      }
    }
  } else if (start[1] === end[1]) {
    const [min, max] =
      start[0] < end[0] ? [start[0], end[0]] : [end[0], start[0]];
    for (let i = min; i <= max; i++) {
      const pt = `${i}, ${start[1]}`;
      if (points[pt]) {
        points[pt] += 1;
      } else {
        points[pt] = 1;
      }
    }
  } else {
    // diagonal
    const incX =
      start[0] < end[0] ? 1 : -1;
    const incY =
      start[1] < end[1] ? 1 : -1;

    for (let i = start[0], j = start[1]; i !== end[0] + incX, j !== end[1] + incY; i += incX, j += incY) {
      const pt = `${i}, ${j}`;
      if (points[pt]) {
        points[pt] += 1;
      } else {
        points[pt] = 1;
      }
    }
  }
});

const moreThanOne = Object.values(points).filter((v) => v > 1).length;

console.log(moreThanOne);

console.timeEnd("logic");
