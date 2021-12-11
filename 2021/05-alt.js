const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./05.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

const parsed = rows.map((row) =>
  row.split(" -> ").map((pair) => pair.split(",").map((v) => parseInt(v, 10)))
);
console.time("logic");

const points = new Uint16Array(10000000);
let moreThanOne = 0;

parsed.forEach(([start, end]) => {
  // diagonal
  const incX = Math.sign(end[0] - start[0]);
  const incY = Math.sign(end[1] - start[1]);

  let i = start[0],
    j = start[1];

  while (true) {
    const pt = i * 10000 + j;
    if (points[pt] === 1) {
      points[pt] += 1;
      moreThanOne++;
    } else if (!points[pt]) {
      points[pt] = 1;
    }

    if (i === end[0] && j === end[1]) {
      break;
    }

    i += incX;
    j += incY;
  }
});

console.log(moreThanOne);

console.timeEnd("logic");
